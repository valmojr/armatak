use arma_rs::Context;
use log::info;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::callbacks::{mavlink_callback_event, mavlink_packet_summary};

pub(crate) struct MavlinkEndpoint {
    pub socket: UdpSocket,
    pub running: Arc<AtomicBool>,
    pub listener: Option<JoinHandle<()>>,
    pub bind_port: u16,
}

static MAVLINK_ENDPOINT: Mutex<Option<MavlinkEndpoint>> = Mutex::new(None);

pub(crate) fn socket_for_send() -> Option<UdpSocket> {
    MAVLINK_ENDPOINT
        .lock()
        .ok()
        .and_then(|endpoint| endpoint.as_ref().and_then(|entry| entry.socket.try_clone().ok()))
}

fn stop_endpoint_internal() {
    let endpoint = MAVLINK_ENDPOINT.lock().unwrap().take();

    if let Some(mut endpoint) = endpoint {
        endpoint.running.store(false, Ordering::Relaxed);
        info!(
            "Stopping MAVLink UDP endpoint on 0.0.0.0:{}",
            endpoint.bind_port
        );

        if let Some(listener) = endpoint.listener.take() {
            let _ = listener.join();
        }
    }
}

pub fn start_endpoint(ctx: Context, bind_port: i32) -> &'static str {
    let bind_port = bind_port.clamp(1, 65535) as u16;

    stop_endpoint_internal();

    let socket = match UdpSocket::bind(format!("0.0.0.0:{bind_port}")) {
        Ok(socket) => socket,
        Err(error) => {
            let _ = ctx.callback_data(
                "MAVLINK UDP ERROR",
                "failed to bind MAVLink UDP endpoint",
                error.to_string(),
            );
            info!(
                "Failed to bind MAVLink UDP endpoint on 0.0.0.0:{}: {}",
                bind_port, error
            );
            return "Failed to bind MAVLink UDP endpoint";
        }
    };

    if let Err(error) = socket.set_read_timeout(Some(Duration::from_millis(500))) {
        info!(
            "Failed to set MAVLink UDP endpoint read timeout on 0.0.0.0:{}: {}",
            bind_port, error
        );
    }

    let listener_socket = match socket.try_clone() {
        Ok(listener_socket) => listener_socket,
        Err(error) => {
            let _ = ctx.callback_data(
                "MAVLINK UDP ERROR",
                "failed to clone MAVLink UDP endpoint socket",
                error.to_string(),
            );
            info!(
                "Failed to clone MAVLink UDP endpoint socket on 0.0.0.0:{}: {}",
                bind_port, error
            );
            return "Failed to clone MAVLink UDP endpoint socket";
        }
    };

    let running = Arc::new(AtomicBool::new(true));
    let listener_running = Arc::clone(&running);

    let listener_ctx = ctx;
    let listener = thread::spawn(move || {
        let mut buffer = [0u8; 2048];
        info!("MAVLink UDP endpoint listening on 0.0.0.0:{}", bind_port);

        while listener_running.load(Ordering::Relaxed) {
            match listener_socket.recv_from(&mut buffer) {
                Ok((received, source)) => {
                    let source_string = source.to_string();
                    info!(
                        "MAVLink UDP endpoint received {} bytes from {}: {}",
                        received,
                        source,
                        mavlink_packet_summary(&buffer[..received])
                    );
                    if let Some(event) = mavlink_callback_event(&buffer[..received], &source_string) {
                        let _ = listener_ctx.callback_data("MAVLINK UDP", event.function, event.data);
                    }
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) => {}
                Err(error) => {
                    if listener_running.load(Ordering::Relaxed) {
                        info!(
                            "MAVLink UDP endpoint listener error on 0.0.0.0:{}: {}",
                            bind_port, error
                        );
                    }
                    break;
                }
            }
        }

        info!(
            "MAVLink UDP endpoint listener stopped on 0.0.0.0:{}",
            bind_port
        );
    });

    *MAVLINK_ENDPOINT.lock().unwrap() = Some(MavlinkEndpoint {
        socket,
        running,
        listener: Some(listener),
        bind_port,
    });

    info!("Started MAVLink UDP endpoint on 0.0.0.0:{}", bind_port);
    "Started MAVLink UDP endpoint"
}

pub fn stop_endpoint(_ctx: Context) -> &'static str {
    stop_endpoint_internal();
    "Stopped MAVLink UDP endpoint"
}
