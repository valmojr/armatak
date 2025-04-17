use arma_rs::Context;
use lazy_static::lazy_static;
use log::info;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::cot;

pub enum TcpCommand {
    SendMessage(String, Context),
    Stop,
}

pub struct TcpClient {
    pub(crate) tx: Sender<TcpCommand>,
}

impl TcpClient {
    pub fn start(&self, address: String, rx: Receiver<TcpCommand>, ctx: Context) {
        if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
            client.stop();
        }

        let connection = Arc::new(Mutex::new(None));
        let connection_clone = Arc::clone(&connection);

        thread::spawn(move || {
            let mut running = true;

            let tcp_thread = thread::spawn(move || match TcpStream::connect(&address) {
                Ok(stream) => {
                    let _ = ctx.callback_data("armatak_tcp_socket", "Connected to TCP Server", address);
                    *connection_clone.lock().unwrap() = Some(stream);
                }
                Err(e) => {
                    let _ = ctx.callback_data(
                        "armatak_tcp_socket_error",
                        "TAK Socket connection failed",
                        e.to_string(),
                    );
                    info!("Failed to connect to TCP server: {}", e);
                }
            });

            while running {
                match rx.recv() {
                    Ok(TcpCommand::SendMessage(message, context)) => {
                        if let Some(mut stream) = connection.lock().unwrap().as_ref() {
                            if let Err(e) = stream.write_all(message.as_bytes()) {
                                info!("Failed to send message: {}", e);

                                let _ = context.callback_data(
                                    "armatak_tcp_socket_error",
                                    "TAK Socket disconnected",
                                    e.to_string(),
                                );

                                running = false;
                            }
                        } else {
                            let _ = context.callback_null(
                                "armatak_tcp_socket_error",
                                "TAK Socket is not active",
                            );
                        }
                    }
                    Ok(TcpCommand::Stop) => {
                        running = false;
                        info!("Stopping TCP client.");
                    }
                    Err(error) => {
                        info!("Error receiving command: {}", error.to_string());
                    }
                }
            }

            tcp_thread.join().unwrap();
        });
    }

    pub fn send_payload(&self, context: Context, payload: String) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            tx.send(TcpCommand::SendMessage(payload, context)).unwrap();
        });
    }

    pub fn stop(&self) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            tx.send(TcpCommand::Stop).unwrap();
        });
    }
}

lazy_static! {
    static ref TCP_CLIENT: Arc<Mutex<Option<TcpClient>>> = Arc::new(Mutex::new(None));
}

pub fn start(ctx: Context, address: String) -> &'static str {
    let (tx, rx): (Sender<TcpCommand>, Receiver<TcpCommand>) = mpsc::channel();

    let client = TcpClient { tx };
    client.start(address, rx, ctx);

    let mut client_guard = TCP_CLIENT.lock().unwrap();
    *client_guard = Some(client);

    "Starting TCP Client"
}

pub fn send_payload(ctx: Context, payload: String) -> &'static str {
    if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
        client.send_payload(ctx, payload);
    } else {
        let _ = ctx.callback_null("armatak_tcp_socket_error", "TCP Client is not running");
        info!("TCP client is not running.");
    }

    "Sending payload to TCP server"
}

pub fn send_eud_cot(ctx: Context, cursor_over_time: cot::eud::EudCoTPayload) -> &'static str {
    let payload = cursor_over_time.to_cot().convert_to_xml();
    send_payload(ctx, payload);

    "Sending End User Device Cursor Over Time to TCP server"
}

pub fn send_marker_cot(ctx: Context, cursor_over_time: cot::nato::MarkerCoTPayload) -> &'static str {
    let payload = cursor_over_time.to_cot().convert_to_xml();
    send_payload(ctx, payload);

    "Sending Marker Cursor Over Time to TCP server"
}

pub fn send_digital_pointer_cot(ctx: Context, cursor_over_time: cot::digital_pointer::DigitalPointerPayload) -> &'static str {
    let payload = cursor_over_time.to_cot().convert_to_xml();
    send_payload(ctx, payload);

    "Sending Digital Pointer Cursor Over Time to TCP server"
}

pub fn stop(ctx: Context) -> &'static str {
    if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
        client.stop();
        let _ = ctx.callback_null("armatak_tcp_socket", "TCP client stopped");
    } else {
        let _ = ctx.callback_null("armatak_tcp_socket_error", "TCP client is not running");
    }

    "Stopping TCP Client"
}
