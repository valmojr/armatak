use arma_rs::Context;
use lazy_static::lazy_static;
use log::info;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result as WsResult, Sender as WsSender};

use crate::structs::{IntoMessage, LocationPayload};

pub enum WsCommand {
    SendMessage(String),
    Stop,
}

enum WsEvent {
    FirstClientConnected,
    LastClientDisconnected,
}

pub struct WsServer {
    pub(crate) tx: Sender<WsCommand>,
}

struct WsHandler {
    out: WsSender,
    clients: Arc<Mutex<Vec<WsSender>>>,
    event_tx: Sender<WsEvent>,
}

impl Handler for WsHandler {
    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        let mut clients = self.clients.lock().unwrap();
        clients.push(self.out.clone());

        let count = clients.len();
        info!("New client connected. Total clients: {}", count);

        if count == 1 {
            let _ = self.event_tx.send(WsEvent::FirstClientConnected);
        }

        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        let mut clients = self.clients.lock().unwrap();
        clients.retain(|client| client.token() != self.out.token());

        let count = clients.len();
        info!("Client disconnected. Total clients: {}", count);

        if count == 0 {
            let _ = self.event_tx.send(WsEvent::LastClientDisconnected);
        }
    }

    fn on_message(&mut self, msg: Message) -> WsResult<()> {
        info!("Received: {}", msg);
        Ok(())
    }
}

impl WsServer {
    pub fn start(&self, ctx: Context, rx: Receiver<WsCommand>) {
        if let Some(ref server) = *WEBSOCKET_SERVER.lock().unwrap() {
            server.stop();
        }

        let clients = Arc::new(Mutex::new(Vec::new()));
        let clients_clone = Arc::clone(&clients);
        let (event_tx, event_rx): (Sender<WsEvent>, Receiver<WsEvent>) = mpsc::channel();

        // Start WebSocket listener in a background thread
        thread::spawn(move || {
            listen("0.0.0.0:4152", |out| WsHandler {
                out,
                clients: Arc::clone(&clients_clone),
                event_tx: event_tx.clone(),
            }).expect("WebSocket server failed to start");
        });

        // This thread owns ctx and reacts to commands and events
        thread::spawn(move || {
            loop {
                // WebSocket message commands
                match rx.try_recv() {
                    Ok(WsCommand::SendMessage(message)) => {
                        let clients_guard = clients.lock().unwrap();
                        for client in clients_guard.iter() {
                            let _ = client.send(message.clone());
                        }
                    }
                    Ok(WsCommand::Stop) => {
                        info!("Stopping WebSocket server.");
                        break;
                    }
                    Err(_) => {}
                }

                // Handle event callbacks with valid Context
                match event_rx.try_recv() {
                    Ok(WsEvent::FirstClientConnected) => {
                        info!("Triggering callback: client_connected");
                        let _ = ctx.callback_null("armatak_websocket", "client_connected");
                    }
                    Ok(WsEvent::LastClientDisconnected) => {
                        info!("Triggering callback: client_disconnected");
                        let _ = ctx.callback_null("armatak_websocket", "client_disconnected");
                    }
                    Err(_) => {}
                }

                thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    }

    pub fn send_message<T: IntoMessage>(&self, payload: T) {
        let message = payload.into_message();
        let _ = self.tx.send(WsCommand::SendMessage(message));
    }

    pub fn stop(&self) {
        let _ = self.tx.send(WsCommand::Stop);
    }
}

lazy_static! {
    static ref WEBSOCKET_SERVER: Arc<Mutex<Option<WsServer>>> = Arc::new(Mutex::new(None));
}

pub fn start(ctx: Context) -> &'static str {
    let (tx, rx): (Sender<WsCommand>, Receiver<WsCommand>) = mpsc::channel();

    let server = WsServer { tx };
    server.start(ctx, rx);

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

    "Sending location to all WebSocket clients"
}

pub fn stop() -> &'static str {
    if let Some(ref server) = *WEBSOCKET_SERVER.lock().unwrap() {
        server.stop();
    } else {
        info!("WebSocket server is not running.");
    }

    "Stopping WebSocket Server"
}
