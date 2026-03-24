use arma_rs::Context;
use log::info;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use super::config::ConnectionConfig;
use super::transport::{connect_stream, TransportStream};
use super::TCP_CLIENT;

pub enum TcpCommand {
    SendMessage(String, Context),
    Stop,
}

pub struct TcpClient {
    pub(crate) tx: Sender<TcpCommand>,
}

impl TcpClient {
    pub fn start(&self, config: ConnectionConfig, rx: Receiver<TcpCommand>, ctx: Context) {
        if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
            client.stop();
        }

        let connection = Arc::new(Mutex::new(None::<TransportStream>));
        let connection_clone = Arc::clone(&connection);

        thread::spawn(move || {
            let mut running = true;
            let connection_message = config.connected_message();

            let tcp_thread = thread::spawn(move || match connect_stream(&config) {
                Ok(stream) => {
                    let target = config.target();
                    let _ = ctx.callback_data("TCP SOCKET", connection_message, target);
                    *connection_clone.lock().unwrap() = Some(stream);
                }
                Err(e) => {
                    let _ = ctx.callback_data(
                        "TCP SOCKET ERROR",
                        "TAK Socket connection failed",
                        e.to_string(),
                    );
                    info!("Failed to connect to TCP server: {}", e);
                }
            });

            while running {
                match rx.recv() {
                    Ok(TcpCommand::SendMessage(message, context)) => {
                        if let Some(stream) = connection.lock().unwrap().as_mut() {
                            if let Err(e) = stream.write_message(message.as_bytes()) {
                                info!("Failed to send message: {}", e);

                                let _ = context.callback_data(
                                    "TCP SOCKET ERROR",
                                    "TAK Socket disconnected",
                                    e.to_string(),
                                );

                                running = false;
                            }
                        } else {
                            let _ = context
                                .callback_null("TCP SOCKET ERROR", "TAK Socket is not active");
                        }
                    }
                    Ok(TcpCommand::Stop) => {
                        running = false;
                        info!("Stopping TCP client.");
                    }
                    Err(error) => {
                        info!("Error receiving command: {}", error);
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
