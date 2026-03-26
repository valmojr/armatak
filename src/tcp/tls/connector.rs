use log::info;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::sync::Arc;

use crate::tcp::transport::TransportStream;

const TCP_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const SOCKET_IO_TIMEOUT: Duration = Duration::from_secs(10);

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

fn load_certificates_from_pem(pem: &str) -> Result<Vec<CertificateDer<'static>>, String> {
    let mut reader = Cursor::new(pem.as_bytes());

    certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("failed to read certs from PEM payload: {}", e))
}

fn load_private_key_from_pem(pem: &str) -> Result<PrivateKeyDer<'static>, String> {
    let mut reader = Cursor::new(pem.as_bytes());

    private_key(&mut reader)
        .map_err(|e| format!("failed to read private key from PEM payload: {}", e))?
        .ok_or_else(|| "no supported private key found in PEM payload".to_string())
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

fn resolve_address(address: &str) -> Result<SocketAddr, String> {
    address
        .to_socket_addrs()
        .map_err(|e| format!("failed to resolve {}: {}", address, e))?
        .next()
        .ok_or_else(|| format!("failed to resolve {}: no socket addresses returned", address))
}

fn connect_tcp(address: &str) -> Result<TcpStream, String> {
    let socket_addr = resolve_address(address)?;
    info!(
        "Opening TCP connection to {} (resolved={}) with timeout {:?}",
        address, socket_addr, TCP_CONNECT_TIMEOUT
    );

    let tcp_stream = TcpStream::connect_timeout(&socket_addr, TCP_CONNECT_TIMEOUT)
        .map_err(|e| format!("failed to connect to {}: {}", address, e))?;

    tcp_stream
        .set_read_timeout(Some(SOCKET_IO_TIMEOUT))
        .map_err(|e| format!("failed to set read timeout on {}: {}", address, e))?;
    tcp_stream
        .set_write_timeout(Some(SOCKET_IO_TIMEOUT))
        .map_err(|e| format!("failed to set write timeout on {}: {}", address, e))?;

    Ok(tcp_stream)
}

pub fn connect_mtls(
    address: &str,
    server_name: &str,
    ca_cert_path: &str,
    client_cert_path: &str,
    client_key_path: &str,
) -> Result<TransportStream, String> {
    info!(
        "Connecting mTLS from file paths to {} using server_name={}",
        address, server_name
    );
    let mut root_store = RootCertStore::empty();
    let ca_certificates = load_certificates(ca_cert_path)?;
    info!(
        "Loaded {} CA certificate(s) from {}",
        ca_certificates.len(),
        ca_cert_path
    );
    for certificate in ca_certificates {
        root_store
            .add(certificate)
            .map_err(|e| format!("failed to add CA certificate from {}: {}", ca_cert_path, e))?;
    }

    let client_certificates = load_certificates(client_cert_path)?;
    info!(
        "Loaded {} client certificate(s) from {}",
        client_certificates.len(),
        client_cert_path
    );
    let client_key = load_private_key(client_key_path)?;
    info!("Loaded client private key from {}", client_key_path);

    let tls_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_client_auth_cert(client_certificates, client_key)
        .map_err(|e| format!("failed to configure mTLS client: {}", e))?;
    info!("Constructed rustls client config for {}", address);

    let tcp_stream = connect_tcp(address)?;
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

    info!("Starting mTLS handshake for {}", address);
    while tls_stream.conn.is_handshaking() {
        tls_stream
            .conn
            .complete_io(&mut tls_stream.sock)
            .map_err(|e| format!("TLS handshake failed: {}", e))?;
    }

    info!("mTLS handshake completed successfully for {}", address);

    Ok(TransportStream::Mtls(tls_stream))
}

pub fn connect_mtls_from_pem(
    address: &str,
    server_name: &str,
    ca_cert_pem: &str,
    client_cert_pem: &str,
    client_key_pem: &str,
) -> Result<TransportStream, String> {
    info!(
        "Connecting mTLS from in-memory PEM payloads to {} using server_name={}",
        address, server_name
    );
    let mut root_store = RootCertStore::empty();
    let ca_certificates = load_certificates_from_pem(ca_cert_pem)?;
    info!(
        "Loaded {} CA certificate(s) from enrollment payload",
        ca_certificates.len()
    );
    for certificate in ca_certificates {
        root_store
            .add(certificate)
            .map_err(|e| format!("failed to add CA certificate from PEM payload: {}", e))?;
    }

    let client_certificates = load_certificates_from_pem(client_cert_pem)?;
    info!(
        "Loaded {} client certificate(s) from enrollment payload",
        client_certificates.len()
    );
    let client_key = load_private_key_from_pem(client_key_pem)?;
    info!("Loaded client private key from enrollment payload");

    let tls_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_client_auth_cert(client_certificates, client_key)
        .map_err(|e| format!("failed to configure mTLS client: {}", e))?;
    info!("Constructed rustls client config for {}", address);

    let tcp_stream = connect_tcp(address)?;
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

    info!("Starting mTLS handshake for {}", address);
    while tls_stream.conn.is_handshaking() {
        tls_stream
            .conn
            .complete_io(&mut tls_stream.sock)
            .map_err(|e| format!("TLS handshake failed: {}", e))?;
    }

    info!("mTLS handshake completed successfully for {}", address);

    Ok(TransportStream::Mtls(tls_stream))
}
