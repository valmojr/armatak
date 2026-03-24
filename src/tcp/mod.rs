use arma_rs::Context;
use lazy_static::lazy_static;
use log::info;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

mod client;
mod config;
mod tls;
mod transport;

pub mod cot;
pub mod draw;

use client::{TcpClient, TcpCommand};
use config::ConnectionConfig;
use tls::artifacts::clear_enrollment_artifacts;

lazy_static! {
    static ref TCP_CLIENT: Arc<Mutex<Option<TcpClient>>> = Arc::new(Mutex::new(None));
}

fn start_with_config(ctx: Context, config: ConnectionConfig) {
    let (tx, rx): (Sender<TcpCommand>, Receiver<TcpCommand>) = mpsc::channel();

    let client = TcpClient { tx };
    client.start(config, rx, ctx);

    let mut client_guard = TCP_CLIENT.lock().unwrap();
    *client_guard = Some(client);
}

pub fn start(ctx: Context, address: String) -> &'static str {
    start_with_config(ctx, ConnectionConfig::Plain { address });

    "Starting TCP Client"
}

pub fn start_mtls(
    ctx: Context,
    address: String,
    server_name: String,
    ca_cert_path: String,
    client_cert_path: String,
    client_key_path: String,
) -> &'static str {
    start_with_config(
        ctx,
        ConnectionConfig::Mtls {
            address,
            server_name,
            ca_cert_path,
            client_cert_path,
            client_key_path,
        },
    );

    "Starting mTLS TCP Client"
}

pub fn start_enroll_mtls(
    ctx: Context,
    host: String,
    server_name: String,
    enroll_port: String,
    username: String,
    password: String,
    client_uid: String,
) -> &'static str {
    clear_enrollment_artifacts();
    start_with_config(
        ctx,
        ConnectionConfig::EnrollMtls {
            host,
            server_name,
            enroll_port,
            username,
            password,
            client_uid,
        },
    );

    "Starting enrolled mTLS TCP Client"
}

pub fn send_payload(ctx: Context, payload: String) -> &'static str {
    if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
        client.send_payload(ctx, payload);
    } else {
        let _ = ctx.callback_null("TCP SOCKET ERROR", "TCP Client is not running");
        info!("TCP client is not running.");
    }

    "Sending payload to TCP server"
}

pub fn stop(ctx: Context) -> &'static str {
    if let Some(ref client) = *TCP_CLIENT.lock().unwrap() {
        client.stop();
        let _ = ctx.callback_null("TCP SOCKET", "TCP client stopped");
        clear_enrollment_artifacts();
    } else {
        let _ = ctx.callback_null("TCP SOCKET ERROR", "TCP client is not running");
    }

    "Stopping TCP Client"
}
