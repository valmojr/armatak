use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use log::info;
use ws::{listen, Message, Result as WsResult};
use lazy_static::lazy_static;

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
                listen("127.0.0.1:3012", |out| {
                    let clients_inner = Arc::clone(&clients_clone);
                    {
                        let mut clients_guard = clients_inner.lock().unwrap();
                        clients_guard.push(out.clone());
                    }
                    
                    move |msg: Message| -> WsResult<()> {
                        info!("Received: {}", msg);
                        Ok(())
                    }
                }).unwrap();
            });

            while running {
                match rx.recv() {
                    Ok(WsCommand::SendMessage(message)) => {
                        let clients_guard = clients.lock().unwrap();
                        for client in clients_guard.iter() {
                            client.send(message.clone()).unwrap();
                        }
                        info!("Broadcasting message: {}", message);
                    }
                    Ok(WsCommand::Stop) => {
                        running = false;
                        info!("Stopping WebSocket server.");
                    }
                    Err(_) => {
                        info!("Error receiving command.");
                    }
                }
            }

            ws_thread.join().unwrap();
        });
    }

    fn send_message(&self, message: String) {
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
        server.send_message(payload);
    } else {
        info!("WebSocket server is not running.");
    }

    "Sending message to all WebSocket clients"
}

pub fn stop() -> &'static str {
    if let Some(ref server) = *WEBSOCKET_SERVER.lock().unwrap() {
        server.stop();
    } else {
        info!("WebSocket server is not running.");
    }

    "Stopping WebSocket server"
}