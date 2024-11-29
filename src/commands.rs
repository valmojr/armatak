use lazy_static::lazy_static;
use log::info;
use std::net::{IpAddr, UdpSocket};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use ws::{listen, Message, Result as WsResult};
use qrcode::{render::unicode::{self}, QrCode};

use crate::structs::{IntoMessage, LocationPayload};

enum WsCommand {
    SendMessage(String),
    Stop,
}

struct WsServer {
    tx: Sender<WsCommand>,
}

impl WsServer {
    fn start(&self, rx: Receiver<WsCommand>) {
        let clients = Arc::new(Mutex::new(Vec::new()));
        let clients_clone = Arc::clone(&clients);

        thread::spawn(move || {
            let mut running = true;

            let ws_thread = thread::spawn(move || {
                listen("0.0.0.0:4152", |out| {
                    let clients_inner = Arc::clone(&clients_clone);
                    {
                        let mut clients_guard = clients_inner.lock().unwrap();
                        clients_guard.push(out.clone());
                    }

                    move |msg: Message| -> WsResult<()> {
                        info!("Received: {}", msg);
                        Ok(())
                    }
                })
                .unwrap();
            });

            while running {
                match rx.recv() {
                    Ok(WsCommand::SendMessage(message)) => {
                        let clients_guard = clients.lock().unwrap();
                        for client in clients_guard.iter() {
                            client.send(message.clone()).unwrap();
                        }
                    }
                    Ok(WsCommand::Stop) => {
                        running = false;
                        info!("Stopping WebSocket server.");
                    }
                    Err(error) => {
                        info!("Error receiving command: {}", error.to_string());
                    }
                }
            }

            ws_thread.join().unwrap();
        });
    }

    fn send_message<T: IntoMessage>(&self, payload: T) {
        let message = payload.into_message();
        self.tx.send(WsCommand::SendMessage(message)).unwrap();
    }

    fn stop(&self) {
        self.tx.send(WsCommand::Stop).unwrap();
    }
}

lazy_static! {
    static ref WEBSOCKET_SERVER: Arc<Mutex<Option<WsServer>>> = Arc::new(Mutex::new(None));
}

pub fn start() -> &'static str {
    let (tx, rx): (Sender<WsCommand>, Receiver<WsCommand>) = mpsc::channel();

    let server = WsServer { tx };
    server.start(rx);

    let mut server_guard = WEBSOCKET_SERVER.lock().unwrap();
    *server_guard = Some(server);

    info!("WebSocket server started.");

    "Starting WebSocket Server"
}

pub fn message(payload: String) -> &'static str {
    if let Some(ref server) = *WEBSOCKET_SERVER.lock().unwrap() {
        info!("Broadcasting message: {}", payload);
        server.send_message(payload);
    } else {
        info!("WebSocket server is not running.");
    }

    "Sending message to all WebSocket clients"
}

pub fn location(payload: LocationPayload) -> &'static str {
    if let Some(ref server) = *WEBSOCKET_SERVER.lock().unwrap() {
        server.send_message(payload);
    } else {
        info!("WebSocket server is not running.");
    }
    "sending location to all WebSocket clients"
}

pub fn stop() -> &'static str {
    if let Some(ref server) = *WEBSOCKET_SERVER.lock().unwrap() {
        server.stop();
    } else {
        info!("WebSocket server is not running.");
    }

    "Stopping WebSocket server"
}

pub fn local_qrcode() -> Vec<String> {
    let mut result = Vec::<String>::new();

    fn get_local_ip() -> Result<IpAddr, String> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
        socket
            .connect("8.8.8.8:80")
            .map_err(|e| e.to_string())?;
        socket
            .local_addr()
            .map(|addr| addr.ip())
            .map_err(|e| e.to_string())
    }

    fn draw_qrcode(data: String) -> String {
        let code = QrCode::new(data).expect("Failed to generate QR Code");
        let ascii_qr = code.render::<unicode::Dense1x2>().quiet_zone(false).build();
        return ascii_qr.replace("\n", ",")
    }

    let parsed_data = get_local_ip();

    match parsed_data {
        Ok(ip) => {
            result.push(draw_qrcode(ip.to_string()));
            result.push(ip.to_string())
        },
        Err(_) => {
            result.push("not provided".to_string());
            result.push("not provided".to_string());
        },
    }

    return result;
}
