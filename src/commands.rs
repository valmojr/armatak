use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use log::{error, info};
use once_cell::sync::Lazy;
use serde_json::json;
use std::{sync::Arc, thread};
use tokio::net::TcpStream;
use tokio::{
    net::TcpListener,
    runtime::Runtime,
    sync::{oneshot, Mutex},
    task::JoinHandle,
};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};

use crate::structs::LocationPayload;

type WebSocket = WebSocketStream<TcpStream>;

struct ServerState {
    handle: Option<JoinHandle<()>>,
    stop_sender: Option<oneshot::Sender<()>>,
    clients: Vec<Arc<Mutex<WebSocket>>>,
}

impl ServerState {
    fn new() -> Self {
        ServerState {
            handle: None,
            stop_sender: None,
            clients: vec![],
        }
    }
}

lazy_static! {
    static ref SERVER_STATE: Arc<Mutex<ServerState>> = Arc::new(Mutex::new(ServerState::new()));
    static ref LOCATION_PAYLOAD: LocationPayload = LocationPayload {
        latitude: 0.0,
        longitude: 0.0,
        altitude: 0.0,
        bearing: 0.0,
    };
}

pub static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Failed to build the Tokio Runtime"));

pub fn start() -> &'static str {
    let server_state = SERVER_STATE.clone();

    thread::spawn(move || {
        RUNTIME.block_on(async {
            let mut state = server_state.lock().await;

            if state.handle.is_some() {
                info!("Server is already running.");
                return;
            }

            let (stop_tx, stop_rx) = oneshot::channel();
            state.stop_sender = Some(stop_tx);

            state.handle = Some(tokio::spawn(async move {
                info!("Starting server...");
                let listener = TcpListener::bind("192.168.0.43:8080").await.expect("Failed to bind");
                info!("WebSocket server running on ws://192.168.0.43:8080");

                tokio::select! {
                    _ = async {
                        while let Ok((stream, _)) = listener.accept().await {
                            let ws_stream = accept_async(stream).await.expect("Failed to accept WebSocket connection");
                            let client = Arc::new(Mutex::new(ws_stream));
                            SERVER_STATE.lock().await.clients.push(client.clone());

                            tokio::spawn(handle_client(client));
                        }
                    } => {}
                    _ = stop_rx => {
                        info!("Shutting down WebSocket server.");
                    }
                }
            }));
        });
    });

    "Server starting..."
}

async fn handle_client(client: Arc<Mutex<WebSocket>>) {
    let mut client = client.lock().await;

    while let Some(Ok(msg)) = client.next().await {
        if let Message::Text(text) = msg {
            info!("Received message from client: {}", text);
        }
    }
}

pub fn send_ping() -> &'static str {
    thread::spawn(move || {
        RUNTIME.block_on(async {
            send_to_all_clients(Message::Text("Ping".into())).await;
        });
    });

    "sending ping..."
}

pub fn send_location() -> &'static str {
    let current_position = LOCATION_PAYLOAD.clone();
    thread::spawn(move || {
        RUNTIME.block_on(async {
            let location_data = json!(current_position);
            send_to_all_clients(Message::Text(location_data.to_string())).await;
        });
    });

    "sending location..."
}

async fn send_to_all_clients(message: Message) {
    info!("Sending message to all clients: {:?}", message);
    let state = SERVER_STATE.lock().await;
    for client in &state.clients {
        let mut client = client.lock().await;
        if let Err(e) = client.send(message.clone()).await {
            error!("Failed to send message: {:?}", e);
        }
    }
}

pub fn stop() -> &'static str {
    let server_state = SERVER_STATE.clone();

    thread::spawn(move || {
        RUNTIME.block_on(async {
            let mut state = server_state.lock().await;

            if let Some(stop_tx) = state.stop_sender.take() {
                let _ = stop_tx.send(());
            }

            if let Some(handle) = state.handle.take() {
                let _ = handle.await;
                state.clients.clear();
                info!("Server stopped.");
            }
        });
    });

    "Server stopping..."
}
