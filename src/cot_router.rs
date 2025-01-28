use log::info;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;

pub enum TcpCommand {
    SendMessage(String),
    Stop,
}

pub struct TcpClient {
    pub(crate) tx: Sender<TcpCommand>,
}

impl TcpClient {
    pub fn start(&self, address: String, rx: Receiver<TcpCommand>) {
        if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
            client.stop();
        }

        let connection = Arc::new(Mutex::new(None));
        let connection_clone = Arc::clone(&connection);

        thread::spawn(move || {
            let mut running = true;

            // TCP connection thread
            let tcp_thread = thread::spawn(move || {
                match TcpStream::connect(&address) {
                    Ok(stream) => {
                        info!("Connected to TCP server at {}", address);
                        *connection_clone.lock().unwrap() = Some(stream);
                    }
                    Err(e) => {
                        info!("Failed to connect to TCP server: {}", e);
                    }
                }
            });

            while running {
                match rx.recv() {
                    Ok(TcpCommand::SendMessage(message)) => {
                        if let Some(mut stream) = connection.lock().unwrap().as_ref() {
                            if let Err(e) = stream.write_all(message.as_bytes()) {
                                info!("Failed to send message: {}", e);
                            }
                        } else {
                            info!("No active TCP connection.");
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

    pub fn send_payload(&self, payload: String) {
        self.tx.send(TcpCommand::SendMessage(payload)).unwrap();
    }

    pub fn stop(&self) {
        self.tx.send(TcpCommand::Stop).unwrap();
    }
}

lazy_static! {
    static ref TCP_CLIENT: Arc<Mutex<Option<TcpClient>>> = Arc::new(Mutex::new(None));
}

pub fn start(address: String) -> &'static str {
    let (tx, rx): (Sender<TcpCommand>, Receiver<TcpCommand>) = mpsc::channel();

    let client = TcpClient { tx };
    client.start(address, rx);

    let mut client_guard = TCP_CLIENT.lock().unwrap();
    *client_guard = Some(client);

    info!("TCP client started.");

    "Starting TCP Client"
}

pub fn send_payload(payload: String) -> &'static str {
    if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
        info!("Sending payload: {}", payload);
        client.send_payload(payload);
    } else {
        info!("TCP client is not running.");
    }

    "Sending payload to TCP server"
}

pub fn stop() -> &'static str {
    if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
        client.stop();
        info!("TCP client stopped.");
    } else {
        info!("TCP client is not running.");
    }

    "Stopping TCP Client"
}
