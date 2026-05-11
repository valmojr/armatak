use arma_rs::Context;
use lazy_static::lazy_static;
use log::info;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref MDNS_CTRL: Mutex<Option<Sender<()>>> = Mutex::new(None);
}

fn detect_local_ipv4() -> Result<Ipv4Addr, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
    socket.connect("8.8.8.8:80").map_err(|e| e.to_string())?;
    match socket.local_addr().map_err(|e| e.to_string())? {
        std::net::SocketAddr::V4(addr) => Ok(*addr.ip()),
        std::net::SocketAddr::V6(_) => Err("Local address is not IPv4".to_string()),
    }
}

fn sanitize_label(value: &str, fallback: &str) -> String {
    let mut sanitized = value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();

    if sanitized.is_empty() {
        sanitized = fallback.to_string();
    }

    if sanitized.len() > 63 {
        sanitized.truncate(63);
    }

    sanitized
}

fn encode_name(name: &str) -> Vec<u8> {
    let mut encoded = Vec::new();
    for label in name.split('.') {
        let bytes = label.as_bytes();
        encoded.push(bytes.len() as u8);
        encoded.extend_from_slice(bytes);
    }
    encoded.push(0);
    encoded
}

fn push_u16(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_be_bytes());
}

fn push_record(buf: &mut Vec<u8>, name: &str, rr_type: u16, rr_class: u16, ttl: u32, rdata: &[u8]) {
    buf.extend_from_slice(&encode_name(name));
    push_u16(buf, rr_type);
    push_u16(buf, rr_class);
    push_u32(buf, ttl);
    push_u16(buf, rdata.len() as u16);
    buf.extend_from_slice(rdata);
}

fn build_mdns_packet(
    instance_name: &str,
    host_name: &str,
    ip: Ipv4Addr,
    port: u16,
    video_uri: &str,
) -> Vec<u8> {
    let service_type = "_mavlink._udp.local";
    let instance_fqdn = format!("{}.{}", instance_name, service_type);
    let host_fqdn = format!("{}.local", host_name);

    let mut packet = Vec::new();
    push_u16(&mut packet, 0);
    push_u16(&mut packet, 0x8400);
    push_u16(&mut packet, 0);
    push_u16(&mut packet, 4);
    push_u16(&mut packet, 0);
    push_u16(&mut packet, 0);

    let ptr_rdata = encode_name(&instance_fqdn);
    push_record(&mut packet, service_type, 12, 0x0001, 120, &ptr_rdata);

    let mut srv_rdata = Vec::new();
    push_u16(&mut srv_rdata, 0);
    push_u16(&mut srv_rdata, 0);
    push_u16(&mut srv_rdata, port);
    srv_rdata.extend_from_slice(&encode_name(&host_fqdn));
    push_record(&mut packet, &instance_fqdn, 33, 0x8001, 120, &srv_rdata);

    let txt_value = format!("uri={}", video_uri);
    let txt_bytes = txt_value.as_bytes();
    let mut txt_rdata = Vec::new();
    txt_rdata.push(txt_bytes.len() as u8);
    txt_rdata.extend_from_slice(txt_bytes);
    push_record(&mut packet, &instance_fqdn, 16, 0x8001, 120, &txt_rdata);

    let a_rdata = ip.octets();
    push_record(&mut packet, &host_fqdn, 1, 0x8001, 120, &a_rdata);

    packet
}

fn stop_existing() {
    if let Ok(mut lock) = MDNS_CTRL.lock() {
        if let Some(tx) = lock.take() {
            let _ = tx.send(());
        }
    }
}

pub fn start_uas_advertisement(
    ctx: Context,
    instance_name: String,
    mavlink_port: i32,
    video_uri: String,
) -> &'static str {
    stop_existing();

    let local_ip = match detect_local_ipv4() {
        Ok(ip) => ip,
        Err(error) => {
            let _ = ctx.callback_data(
                "MDNS ERROR",
                "Failed to determine local IPv4",
                error.clone(),
            );
            return "mdns local IPv4 error";
        }
    };

    let port = mavlink_port.clamp(1, 65535) as u16;
    let safe_instance = sanitize_label(&instance_name, "ArmaTAK-UAS");
    let host_label = sanitize_label(
        &format!("armatak-{}", safe_instance.to_lowercase()),
        "armatak-uas-host",
    );
    let packet = build_mdns_packet(&safe_instance, &host_label, local_ip, port, &video_uri);
    let callback_video_uri = video_uri.clone();
    let multicast_addr = SocketAddrV4::new(Ipv4Addr::new(224, 0, 0, 251), 5353);

    let (stop_tx, stop_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    if let Ok(mut lock) = MDNS_CTRL.lock() {
        *lock = Some(stop_tx);
    }

    thread::spawn(move || {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket) => socket,
            Err(error) => {
                info!("mDNS failed to bind UDP socket: {}", error);
                return;
            }
        };

        let _ = socket.set_multicast_ttl_v4(255);
        let _ = socket.set_multicast_loop_v4(true);

        info!(
            "Starting mDNS UAS advertisement instance={} host={} ip={} port={} video_uri={}",
            safe_instance, host_label, local_ip, port, video_uri
        );

        loop {
            match socket.send_to(&packet, multicast_addr) {
                Ok(size) => info!(
                    "Sent mDNS UAS advertisement ({} bytes) to {}",
                    size, multicast_addr
                ),
                Err(error) => info!("Failed sending mDNS UAS advertisement: {}", error),
            }

            match stop_rx.recv_timeout(Duration::from_secs(5)) {
                Ok(_) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(_) => break,
            }
        }

        info!(
            "Stopped mDNS UAS advertisement for instance={}",
            safe_instance
        );
    });

    let _ = ctx.callback_data(
        "MDNS",
        "UAS advertisement started",
        format!("{}:{} | {}", local_ip, port, callback_video_uri),
    );

    "starting mdns uas advertisement"
}

pub fn stop(ctx: Context) -> &'static str {
    let had_running = match MDNS_CTRL.lock() {
        Ok(mut lock) => {
            if let Some(tx) = lock.take() {
                let _ = tx.send(());
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };

    if had_running {
        let _ = ctx.callback_null("MDNS", "UAS advertisement stopped");
        "stopping mdns advertisement"
    } else {
        let _ = ctx.callback_null("MDNS ERROR", "No mDNS advertisement is running");
        "no mdns advertisement running"
    }
}
