use rustls::{ClientConnection, StreamOwned};
use std::io::Write;
use std::net::TcpStream;

use super::config::ConnectionConfig;
use super::tls::{connect_mtls, enroll_and_connect};

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

pub fn connect_stream(config: &ConnectionConfig) -> Result<TransportStream, String> {
    match config {
        ConnectionConfig::Plain { address } => TcpStream::connect(address)
            .map(TransportStream::Plain)
            .map_err(|e| format!("failed to connect to {}: {}", address, e)),
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
