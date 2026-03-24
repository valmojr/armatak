use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::Arc;

use crate::tcp::transport::TransportStream;

fn load_certificates(path: &str) -> Result<Vec<CertificateDer<'static>>, String> {
    let file = File::open(path).map_err(|e| format!("failed to open cert file {}: {}", path, e))?;
    let mut reader = BufReader::new(file);

    certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("failed to read certs from {}: {}", path, e))
}

fn load_private_key(path: &str) -> Result<PrivateKeyDer<'static>, String> {
    let file = File::open(path).map_err(|e| format!("failed to open key file {}: {}", path, e))?;
    let mut reader = BufReader::new(file);

    private_key(&mut reader)
        .map_err(|e| format!("failed to read private key from {}: {}", path, e))?
        .ok_or_else(|| format!("no supported private key found in {}", path))
}

fn infer_server_name(address: &str) -> &str {
    address
        .trim()
        .trim_start_matches('[')
        .split(']')
        .next()
        .unwrap_or(address)
        .split(':')
        .next()
        .unwrap_or(address)
}

pub fn connect_mtls(
    address: &str,
    server_name: &str,
    ca_cert_path: &str,
    client_cert_path: &str,
    client_key_path: &str,
) -> Result<TransportStream, String> {
    let mut root_store = RootCertStore::empty();
    for certificate in load_certificates(ca_cert_path)? {
        root_store
            .add(certificate)
            .map_err(|e| format!("failed to add CA certificate from {}: {}", ca_cert_path, e))?;
    }

    let client_certificates = load_certificates(client_cert_path)?;
    let client_key = load_private_key(client_key_path)?;

    let tls_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_client_auth_cert(client_certificates, client_key)
        .map_err(|e| format!("failed to configure mTLS client: {}", e))?;

    let tcp_stream = TcpStream::connect(address)
        .map_err(|e| format!("failed to connect to {}: {}", address, e))?;
    let resolved_server_name = if server_name.trim().is_empty() {
        infer_server_name(address).to_string()
    } else {
        server_name.trim().to_string()
    };

    let server_name = ServerName::try_from(resolved_server_name.clone())
        .map_err(|_| format!("invalid TLS server name: {}", resolved_server_name))?;
    let mut tls_stream = StreamOwned::new(
        ClientConnection::new(Arc::new(tls_config), server_name)
            .map_err(|e| format!("failed to create TLS client: {}", e))?,
        tcp_stream,
    );

    while tls_stream.conn.is_handshaking() {
        tls_stream
            .conn
            .complete_io(&mut tls_stream.sock)
            .map_err(|e| format!("TLS handshake failed: {}", e))?;
    }

    Ok(TransportStream::Mtls(tls_stream))
}
