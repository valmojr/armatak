use log::info;
use rustls::{ClientConnection, StreamOwned};
use std::io::Write;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

use super::config::ConnectionConfig;
use super::tls::{connect_mtls, enroll_and_connect};

const TCP_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const SOCKET_IO_TIMEOUT: Duration = Duration::from_secs(10);

pub enum TransportStream {
    Plain(TcpStream),
    Mtls(StreamOwned<ClientConnection, TcpStream>),
}

impl TransportStream {
    pub fn write_message(&mut self, message: &[u8]) -> Result<(), std::io::Error> {
        match self {
            Self::Plain(stream) => {
                stream.write_all(message)?;
                stream.flush()
            }
            Self::Mtls(stream) => {
                stream.write_all(message)?;
                stream.flush()
            }
        }
    }
}

fn connect_plain(address: &str) -> Result<TransportStream, String> {
    let socket_addr: SocketAddr = address
        .to_socket_addrs()
        .map_err(|e| format!("failed to resolve {}: {}", address, e))?
        .next()
        .ok_or_else(|| format!("failed to resolve {}: no socket addresses returned", address))?;

    info!(
        "Opening plain TCP connection to {} (resolved={}) with timeout {:?}",
        address, socket_addr, TCP_CONNECT_TIMEOUT
    );

    let stream = TcpStream::connect_timeout(&socket_addr, TCP_CONNECT_TIMEOUT)
        .map_err(|e| format!("failed to connect to {}: {}", address, e))?;
    stream
        .set_read_timeout(Some(SOCKET_IO_TIMEOUT))
        .map_err(|e| format!("failed to set read timeout on {}: {}", address, e))?;
    stream
        .set_write_timeout(Some(SOCKET_IO_TIMEOUT))
        .map_err(|e| format!("failed to set write timeout on {}: {}", address, e))?;

    Ok(TransportStream::Plain(stream))
}

pub fn connect_stream(config: &ConnectionConfig) -> Result<TransportStream, String> {
    info!("connect_stream invoked for {}", config.describe());
    match config {
        ConnectionConfig::Plain { address } => connect_plain(address),
        ConnectionConfig::Mtls {
            address,
            server_name,
            ca_cert_path,
            client_cert_path,
            client_key_path,
        } => connect_mtls(
            address,
            server_name,
            ca_cert_path,
            client_cert_path,
            client_key_path,
        ),
        ConnectionConfig::EnrollMtls {
            host,
            server_name,
            enroll_port,
            username,
            password,
            client_uid,
        } => enroll_and_connect(
            host,
            server_name,
            enroll_port,
            username,
            password,
            client_uid,
        ),
    }
}
