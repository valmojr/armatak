use std::{sync::Arc, thread};
use log::{error, info};
use tokio::{net::TcpListener, sync::{Mutex, oneshot}, task::JoinHandle, runtime::Runtime};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use serde_json::json;
use lazy_static::lazy_static;

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
}

fn create_tokio_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
}

pub fn start() -> &'static str {
    let rt = create_tokio_runtime();
    let server_state = SERVER_STATE.clone();

    thread::spawn(move || {
        rt.block_on(async {
            let mut state = server_state.lock().await;

            if state.handle.is_some() {
                info!("Server is already running.");
                return;
            }

            let (stop_tx, stop_rx) = oneshot::channel();
            state.stop_sender = Some(stop_tx);

            state.handle = Some(tokio::spawn(async move {
                info!("Starting server...");
                let listener = TcpListener::bind("192.168.15.8:8080").await.expect("Failed to bind");
                info!("WebSocket server running on ws://192.168.15.8:8080");

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
        let rt = create_tokio_runtime();
        rt.block_on(async {
            send_to_all_clients(Message::Text("Ping".into())).await;
        });
    });

    "sending ping..."
}

pub fn send_location() -> &'static str {
    thread::spawn(move || {
        let rt = create_tokio_runtime();
        rt.block_on(async {
            let location_data = json!({ "location": "42.3601, -71.0589" });
            send_to_all_clients(Message::Text(location_data.to_string())).await;
        });
    });

    "sending location..."
}

async fn send_to_all_clients(message: Message) {
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
        let rt = create_tokio_runtime();
        rt.block_on(async {
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