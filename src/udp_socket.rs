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

pub fn send_payload(ctx: Context, payload: String) -> &'static str {
    if let Some(ref client) = *UDP_CLIENT.lock().unwrap() {
        client.send_payload(ctx, payload);
    } else {
        let _ = ctx.callback_null("UDP SOCKET ERROR", "UDP Socket is not running");
        info!("UDP send requested while socket was not running");
    }

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

pub fn stop(ctx: Context) -> &'static str {
    if let Some(ref client) = *UDP_CLIENT.lock().unwrap() {
        info!("UDP socket stop requested for {}", client.address);
        client.stop();
        let _ = ctx.callback_null("UDP SOCKET", "EUD Disconnected");
    } else {
        let _ = ctx.callback_null("UDP SOCKET ERROR", "UDP Socket is not running");
        info!("UDP stop requested while socket was not running");
    }

    "Stopping UDP Client"
}
