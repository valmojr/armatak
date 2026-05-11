use arma_rs::Context;
use log::{info, warn};
use std::collections::VecDeque;
use std::panic::{self, AssertUnwindSafe};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

use super::config::ConnectionConfig;
use super::transport::{connect_stream, TransportStream};
use super::TCP_CLIENT;

const CONNECT_POLL_INTERVAL: Duration = Duration::from_millis(200);
const MAX_PENDING_MESSAGES: usize = 128;

pub enum TcpCommand {
    SendMessage(String, Context),
    Stop,
}

pub struct TcpClient {
    pub(crate) tx: Sender<TcpCommand>,
}

enum ConnectionState {
    Connecting,
    Connected,
    Failed(String),
}

enum ConnectEvent {
    Connected(TransportStream),
    Failed(String),
}

fn describe_panic_payload(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "unknown panic payload".to_string()
    }
}

fn log_message_preview(message: &str) -> String {
    message.chars().take(96).collect::<String>()
}

fn send_over_stream(
    stream: &mut TransportStream,
    context: &Context,
    message: String,
) -> Result<(), String> {
    let message_len = message.len();
    info!("Sending TCP payload ({} bytes)", message_len);
    stream.write_message(message.as_bytes()).map_err(|e| {
        let message = e.to_string();
        let _ = context.callback_data(
            "TCP SOCKET ERROR",
            "TAK Socket disconnected",
            message.clone(),
        );
        message
    })
}

fn flush_pending_messages(
    connection: &mut Option<TransportStream>,
    pending_messages: &mut VecDeque<(String, Context)>,
    state: &mut ConnectionState,
) {
    if pending_messages.is_empty() {
        return;
    }

    let Some(stream) = connection.as_mut() else {
        return;
    };

    info!(
        "Flushing {} queued TCP payload(s) after connection became active",
        pending_messages.len()
    );

    while let Some((message, context)) = pending_messages.pop_front() {
        if let Err(error) = send_over_stream(stream, &context, message) {
            info!("Failed to send queued message: {}", error);
            *state = ConnectionState::Failed(error);
            *connection = None;
            return;
        }
    }
}

fn poll_connect_event(
    connect_rx: &Receiver<ConnectEvent>,
    connection: &mut Option<TransportStream>,
    state: &mut ConnectionState,
    pending_messages: &mut VecDeque<(String, Context)>,
    ctx: &Context,
    connection_message: &str,
    target: &str,
) {
    loop {
        match connect_rx.try_recv() {
            Ok(ConnectEvent::Connected(stream)) => {
                info!("TCP connection established successfully: {}", target);
                let _ = ctx.callback_data("TCP SOCKET", connection_message, target.to_string());
                *connection = Some(stream);
                *state = ConnectionState::Connected;
                flush_pending_messages(connection, pending_messages, state);
            }
            Ok(ConnectEvent::Failed(error)) => {
                info!("Failed to connect to TCP server: {}", error);
                let _ = ctx.callback_data(
                    "TCP SOCKET ERROR",
                    "TAK Socket connection failed",
                    error.clone(),
                );
                *state = ConnectionState::Failed(error);
            }
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => break,
        }
    }
}

impl TcpClient {
    pub fn start(&self, config: ConnectionConfig, rx: Receiver<TcpCommand>, ctx: Context) {
        if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
            info!("Existing TCP client detected; stopping previous instance before restart.");
            client.stop();
        }

        thread::spawn(move || {
            let mut running = true;
            let connection_message = config.connected_message();
            let config_description = config.describe();
            let target = config.target();
            let mut state = ConnectionState::Connecting;
            let mut connection: Option<TransportStream> = None;
            let mut pending_messages: VecDeque<(String, Context)> = VecDeque::new();
            let (connect_tx, connect_rx) = mpsc::channel();

            info!(
                "TCP worker thread started with config: {}",
                config_description
            );

            let tcp_thread = thread::spawn(move || {
                let connect_result =
                    panic::catch_unwind(AssertUnwindSafe(|| connect_stream(&config)));

                match connect_result {
                    Ok(Ok(stream)) => {
                        let _ = connect_tx.send(ConnectEvent::Connected(stream));
                    }
                    Ok(Err(error)) => {
                        let _ = connect_tx.send(ConnectEvent::Failed(error));
                    }
                    Err(payload) => {
                        let message = format!(
                            "TCP connection worker panicked: {}",
                            describe_panic_payload(payload)
                        );
                        let _ = connect_tx.send(ConnectEvent::Failed(message));
                    }
                }
            });

            while running {
                poll_connect_event(
                    &connect_rx,
                    &mut connection,
                    &mut state,
                    &mut pending_messages,
                    &ctx,
                    connection_message,
                    &target,
                );

                match rx.recv_timeout(CONNECT_POLL_INTERVAL) {
                    Ok(TcpCommand::SendMessage(message, context)) => {
                        let message_len = message.len();
                        match &mut state {
                            ConnectionState::Connected => {
                                if let Some(stream) = connection.as_mut() {
                                    if let Err(error) = send_over_stream(stream, &context, message)
                                    {
                                        info!("Failed to send message: {}", error);
                                        state = ConnectionState::Failed(error);
                                        connection = None;
                                    }
                                } else {
                                    warn!(
                                        "Connection state said connected, but no socket was present; queuing payload."
                                    );
                                    pending_messages.push_back((message, context));
                                }
                            }
                            ConnectionState::Connecting => {
                                if pending_messages.len() >= MAX_PENDING_MESSAGES {
                                    let preview = log_message_preview(&message);
                                    warn!(
                                        "Dropping TCP payload because connection is still pending and queue is full ({} bytes, preview={:?})",
                                        message_len, preview
                                    );
                                    let _ = context.callback_data(
                                        "TCP SOCKET ERROR",
                                        "TAK Socket is still connecting",
                                        format!(
                                            "queue full while connecting; dropped payload ({} bytes, preview={:?})",
                                            message_len, preview
                                        ),
                                    );
                                } else {
                                    info!(
                                        "Queueing TCP payload while connection is pending ({} bytes, queued={})",
                                        message_len,
                                        pending_messages.len() + 1
                                    );
                                    pending_messages.push_back((message, context));
                                }
                            }
                            ConnectionState::Failed(error) => {
                                let preview = log_message_preview(&message);
                                warn!(
                                    "Dropping TCP payload because connection is in failed state ({} bytes, preview={:?}, error={})",
                                    message_len, preview, error
                                );
                                let _ = context.callback_data(
                                    "TCP SOCKET ERROR",
                                    "TAK Socket is not connected",
                                    error.clone(),
                                );
                            }
                        }
                    }
                    Ok(TcpCommand::Stop) => {
                        running = false;
                        info!("Stopping TCP client.");
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => {
                        warn!("TCP command channel disconnected.");
                        running = false;
                    }
                }
            }

            info!("Waiting for TCP connection thread to finish.");
            match tcp_thread.join() {
                Ok(()) => info!("TCP connection thread joined successfully."),
                Err(payload) => warn!(
                    "TCP connection thread join reported a panic: {}",
                    describe_panic_payload(payload)
                ),
            }
            info!("TCP worker thread finished.");
        });
    }

    pub fn send_payload(&self, context: Context, payload: String) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            info!("Dispatching queued TCP payload command.");
            if let Err(error) = tx.send(TcpCommand::SendMessage(payload, context)) {
                warn!("Failed to dispatch TCP payload command: {}", error);
            }
        });
    }

    pub fn stop(&self) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            info!("Dispatching TCP stop command.");
            if let Err(error) = tx.send(TcpCommand::Stop) {
                warn!("Failed to dispatch TCP stop command: {}", error);
            }
        });
    }
}
