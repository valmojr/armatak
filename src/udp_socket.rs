use arma_rs::Context;
use lazy_static::lazy_static;
use log::info;
use std::net::UdpSocket;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::cot;

pub enum UdpCommand {
    SendMessage(String, Context),
    Stop,
}

pub struct UdpClient {
    pub(crate) tx: Sender<UdpCommand>,
    pub(crate) address: String,
}

impl UdpClient {
    pub fn start(&self, address: String, rx: Receiver<UdpCommand>, ctx: Context) {
        thread::spawn(move || {
            info!("Starting UDP client thread for destination {}", address);

            let socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(s) => s,
                Err(e) => {
                    let _ = ctx.callback_data(
                        "UDP SOCKET ERROR",
                        "Failed to bind UDP socket",
                        e.to_string(),
                    );
                    info!("Failed to bind UDP socket for {}: {}", address, e);
                    return;
                }
            };

            if let Ok(local_addr) = socket.local_addr() {
                info!(
                    "UDP client bound local socket {} for destination {}",
                    local_addr, address
                );
            }

            let _ = ctx.callback_data("UDP SOCKET", "EUD Connected", address.clone());
            info!("UDP client reported EUD Connected for {}", address);

            let mut running = true;
            while running {
                match rx.recv() {
                    Ok(UdpCommand::SendMessage(message, context)) => {
                        info!("UDP client sending {} bytes to {}", message.len(), address);
                        if let Err(e) = socket.send_to(message.as_bytes(), &address) {
                            info!("Failed to send UDP message to {}: {}", address, e);
                            let _ = context.callback_data(
                                "UDP SOCKET ERROR",
                                "Failed to send UDP message",
                                e.to_string(),
                            );
                        }
                    }
                    Ok(UdpCommand::Stop) => {
                        running = false;
                        info!("Stopping UDP client for {}", address);
                    }
                    Err(error) => {
                        info!("Error receiving UDP command for {}: {}", address, error);
                    }
                }
            }

            info!("UDP client thread exited for {}", address);
        });
    }

    pub fn send_payload(&self, context: Context, payload: String) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            tx.send(UdpCommand::SendMessage(payload, context)).unwrap();
        });
    }

    pub fn stop(&self) {
        let tx = self.tx.clone();
        let address = self.address.clone();
        thread::spawn(move || {
            info!("Queueing stop for UDP client {}", address);
            tx.send(UdpCommand::Stop).unwrap();
        });
    }
}

lazy_static! {
    static ref UDP_CLIENT: Arc<Mutex<Option<UdpClient>>> = Arc::new(Mutex::new(None));
    static ref LRF_CLIENT: Arc<Mutex<Option<UdpClient>>> = Arc::new(Mutex::new(None));
    static ref COT_CLIENT: Arc<Mutex<Option<UdpClient>>> = Arc::new(Mutex::new(None));
}

pub fn start(ctx: Context, address: String) -> &'static str {
    info!("UDP socket start requested for {}", address);

    let (tx, rx): (Sender<UdpCommand>, Receiver<UdpCommand>) = mpsc::channel();

    let client = UdpClient {
        tx,
        address: address.clone(),
    };

    {
        let mut client_guard = UDP_CLIENT.lock().unwrap();
        if let Some(ref existing_client) = *client_guard {
            info!(
                "Stopping previous UDP client {} before starting {}",
                existing_client.address, address
            );
            existing_client.stop();
        }
        *client_guard = Some(UdpClient {
            tx: client.tx.clone(),
            address: client.address.clone(),
        });
    }

    client.start(address, rx, ctx);

    "Starting UDP Client"
}

fn send_with_client(
    client_slot: &Arc<Mutex<Option<UdpClient>>>,
    ctx: Context,
    payload: String,
    missing_message: &'static str,
) {
    if let Some(ref client) = *client_slot.lock().unwrap() {
        client.send_payload(ctx, payload);
    } else {
        let _ = ctx.callback_null("UDP SOCKET ERROR", missing_message);
        info!("UDP send requested while target socket was not running");
    }
}

pub fn send_payload(ctx: Context, payload: String) -> &'static str {
    send_with_client(&UDP_CLIENT, ctx, payload, "UDP Socket is not running");

    "Sending payload to UDP server"
}

pub fn send_gps_cot(
    ctx: Context,
    cursor_over_time: cot::gps::ExternalPositionPayload,
) -> &'static str {
    let payload = cursor_over_time.to_cot().convert_to_xml();
    send_payload(ctx, payload);

    "Sending GPS Cursor Over Time to UDP server"
}

pub fn send_eud_cot(
    ctx: Context,
    cursor_over_time: cot::gps::ExternalPositionPayload,
) -> &'static str {
    send_with_client(
        &COT_CLIENT,
        ctx,
        cursor_over_time.to_cot().convert_to_xml(),
        "CoT UDP Socket is not running",
    );

    "Sending EUD Cursor Over Time to CoT UDP server"
}

pub fn start_lrf(ctx: Context, address: String) -> &'static str {
    info!("LRF UDP socket start requested for {}", address);

    let (tx, rx): (Sender<UdpCommand>, Receiver<UdpCommand>) = mpsc::channel();

    let client = UdpClient {
        tx,
        address: address.clone(),
    };

    {
        let mut client_guard = LRF_CLIENT.lock().unwrap();
        if let Some(ref existing_client) = *client_guard {
            info!(
                "Stopping previous LRF UDP client {} before starting {}",
                existing_client.address, address
            );
            existing_client.stop();
        }
        *client_guard = Some(UdpClient {
            tx: client.tx.clone(),
            address: client.address.clone(),
        });
    }

    client.start(address, rx, ctx);

    "Starting LRF UDP Client"
}

pub fn start_cot(ctx: Context, address: String) -> &'static str {
    info!("CoT UDP socket start requested for {}", address);

    let (tx, rx): (Sender<UdpCommand>, Receiver<UdpCommand>) = mpsc::channel();

    let client = UdpClient {
        tx,
        address: address.clone(),
    };

    {
        let mut client_guard = COT_CLIENT.lock().unwrap();
        if let Some(ref existing_client) = *client_guard {
            info!(
                "Stopping previous CoT UDP client {} before starting {}",
                existing_client.address, address
            );
            existing_client.stop();
        }
        *client_guard = Some(UdpClient {
            tx: client.tx.clone(),
            address: client.address.clone(),
        });
    }

    client.start(address, rx, ctx);

    "Starting CoT UDP Client"
}

pub fn send_lrf(ctx: Context, payload: cot::lrf::LaserRangeFinderPayload) -> &'static str {
    send_with_client(
        &LRF_CLIENT,
        ctx,
        payload.to_lrf_message(),
        "LRF UDP Socket is not running",
    );

    "Sending Laser Range Finder payload to UDP server"
}

pub fn clear_lrf(ctx: Context, payload: cot::lrf::LaserRangeFinderClearPayload) -> &'static str {
    send_with_client(
        &LRF_CLIENT,
        ctx,
        payload.to_lrf_message(),
        "LRF UDP Socket is not running",
    );

    "Clearing Laser Range Finder payload on UDP server"
}

pub fn send_digital_pointer_cot(
    ctx: Context,
    payload: cot::digital_pointer::DigitalPointerPayload,
) -> &'static str {
    send_with_client(
        &COT_CLIENT,
        ctx,
        payload.to_cot().convert_to_xml(),
        "CoT UDP Socket is not running",
    );

    "Sending Digital Pointer CoT to UDP server"
}

pub fn stop(ctx: Context) -> &'static str {
    if let Some(ref client) = *UDP_CLIENT.lock().unwrap() {
        info!("UDP socket stop requested for {}", client.address);
        client.stop();
        let _ = ctx.callback_null("UDP SOCKET", "EUD Disconnected");
    } else {
        let _ = ctx.callback_null("UDP SOCKET ERROR", "UDP Socket is not running");
        info!("UDP stop requested while socket was not running");
    }

    if let Some(ref client) = *LRF_CLIENT.lock().unwrap() {
        info!("LRF UDP socket stop requested for {}", client.address);
        client.stop();
    }

    if let Some(ref client) = *COT_CLIENT.lock().unwrap() {
        info!("CoT UDP socket stop requested for {}", client.address);
        client.stop();
    }

    "Stopping UDP Client"
}
