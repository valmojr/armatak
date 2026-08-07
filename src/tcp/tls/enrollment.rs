use log::info;
use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair, PKCS_RSA_SHA256};
use reqwest::blocking::Client;
use serde::Deserialize;
use uuid::Uuid;

use super::connector::connect_mtls_from_pem;
use super::protocol::{
    enrollment_config_url, enrollment_sign_url, normalize_certificate_pem,
    parse_enrollment_config, EnrollmentConfig,
};
use crate::tcp::transport::TransportStream;

#[derive(Deserialize)]
struct EnrollmentResponse {
    #[serde(rename = "signedCert")]
    signed_cert: String,
    ca0: String,
}

fn enrollment_http_client() -> Result<Client, String> {
    Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("failed to build enrollment HTTP client: {}", e))
}

fn response_error_details(response: reqwest::blocking::Response) -> String {
    let status = response.status();
    match response.text() {
        Ok(body) => {
            let trimmed = body.trim();
            if trimmed.is_empty() {
                status.to_string()
            } else {
                format!("{}: {}", status, trimmed)
            }
        }
        Err(_) => status.to_string(),
    }
}

fn fetch_enrollment_config(
    host: &str,
    enroll_port: &str,
    username: &str,
    password: &str,
) -> Result<EnrollmentConfig, String> {
    let url = enrollment_config_url(host, enroll_port);
    info!("Fetching TAK enrollment config from {}", url);

    let response = enrollment_http_client()?
        .get(&url)
        .basic_auth(username.trim(), Some(password.to_string()))
        .send()
        .map_err(|e| format!("failed to fetch {}: {}", url, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "failed to fetch {}: {}",
            url,
            response_error_details(response)
        ));
    }

    let response_text = response
        .text()
        .map_err(|e| format!("failed to read config response from {}: {}", url, e))?;
    let config = parse_enrollment_config(&response_text);

    info!(
        "Enrollment config received: server_port={} enroll_path={}",
        config.server_port, config.enroll_path
    );

    Ok(config)
}

fn enroll_client_certificate(
    host: &str,
    enroll_port: &str,
    enroll_path: &str,
    username: &str,
    password: &str,
    client_uid: &str,
) -> Result<(String, String, String), String> {
    info!(
        "Generating RSA client keypair and CSR for enrolled TAK client {}",
        client_uid
    );
    let key_pair = KeyPair::generate_for(&PKCS_RSA_SHA256)
        .map_err(|e| format!("failed to generate client keypair: {}", e))?;

    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(DnType::CommonName, client_uid);
    distinguished_name.push(DnType::OrganizationName, "ArmaTAK");
    distinguished_name.push(DnType::OrganizationalUnitName, "ArmaTAK Session");

    let mut params = CertificateParams::new(vec![])
        .map_err(|e| format!("failed to create CSR params: {}", e))?;
    params.distinguished_name = distinguished_name;

    let csr = params
        .serialize_request(&key_pair)
        .map_err(|e| format!("failed to generate CSR: {}", e))?;
    let csr_der = csr.der().as_ref().to_vec();

    let url = enrollment_sign_url(host, enroll_port, enroll_path, client_uid);
    info!(
        "Submitting client certificate enrollment request for {} to {}",
        client_uid, url
    );

    let response = enrollment_http_client()?
        .post(&url)
        .basic_auth(username.trim(), Some(password.to_string()))
        .header("Accept", "application/json")
        .header("Content-Type", "application/pkcs10")
        .body(csr_der)
        .send()
        .map_err(|e| format!("failed to enroll client certificate at {}: {}", url, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "failed to enroll client certificate at {}: {}",
            url,
            response_error_details(response)
        ));
    }

    let enrollment: EnrollmentResponse = response
        .json()
        .map_err(|e| format!("failed to parse enrollment response: {}", e))?;
    info!(
        "Enrollment response parsed successfully for {} (signed_cert_len={}, ca_len={})",
        client_uid,
        enrollment.signed_cert.len(),
        enrollment.ca0.len()
    );

    let ca_cert_pem = normalize_certificate_pem(&enrollment.ca0);
    let cert_pem = normalize_certificate_pem(&enrollment.signed_cert);
    let key_pem = key_pair.serialize_pem();

    Ok((ca_cert_pem, cert_pem, key_pem))
}

pub fn enroll_and_connect(
    host: &str,
    server_name: &str,
    enroll_port: &str,
    username: &str,
    password: &str,
    client_uid: &str,
) -> Result<TransportStream, String> {
    let normalized_client_uid = if client_uid.trim().is_empty() {
        format!("armatak-{}", Uuid::new_v4())
    } else {
        client_uid.trim().to_string()
    };
    info!(
        "Starting enroll_and_connect for host={} enroll_port={} server_name={} client_uid={}",
        host, enroll_port, server_name, normalized_client_uid
    );

    let enrollment_config = fetch_enrollment_config(host, enroll_port, username, password)?;
    let (ca_cert_pem, client_cert_pem, client_key_pem) = enroll_client_certificate(
        host,
        enroll_port,
        &enrollment_config.enroll_path,
        username,
        password,
        &normalized_client_uid,
    )?;

    connect_mtls_from_pem(
        &format!("{}:{}", host.trim(), enrollment_config.server_port.trim()),
        if server_name.trim().is_empty() {
            host.trim()
        } else {
            server_name.trim()
        },
        &ca_cert_pem,
        &client_cert_pem,
        &client_key_pem,
    )
}
