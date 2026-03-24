use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair, PKCS_ECDSA_P256_SHA256};
use reqwest::blocking::Client;
use serde::Deserialize;
use uuid::Uuid;

use super::artifacts::{
    persist_enrollment_artifacts, store_enrollment_artifacts, EnrollmentArtifacts,
};
use super::connector::connect_mtls;
use crate::tcp::transport::TransportStream;

#[derive(Deserialize)]
struct EnrollmentResponse {
    #[serde(rename = "signedCert")]
    signed_cert: String,
    ca0: String,
}

struct EnrollmentConfig {
    server_port: String,
    enroll_path: String,
}

fn extract_tag_value(xml: &str, tag_name: &str) -> Option<String> {
    let open_tag = format!("<{}>", tag_name);
    let close_tag = format!("</{}>", tag_name);
    let start = xml.find(&open_tag)? + open_tag.len();
    let end = xml[start..].find(&close_tag)? + start;
    Some(xml[start..end].trim().to_string())
}

fn wrap_pem_body(base64_body: &str, begin: &str, end: &str) -> String {
    let mut wrapped = String::new();
    let normalized = base64_body.trim().replace(['\r', '\n'], "");

    wrapped.push_str(begin);
    wrapped.push('\n');
    for chunk in normalized.as_bytes().chunks(64) {
        wrapped.push_str(std::str::from_utf8(chunk).unwrap_or_default());
        wrapped.push('\n');
    }
    wrapped.push_str(end);
    wrapped.push('\n');
    wrapped
}

fn enrollment_http_client() -> Result<Client, String> {
    Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("failed to build enrollment HTTP client: {}", e))
}

fn fetch_enrollment_config(host: &str, enroll_port: &str) -> Result<EnrollmentConfig, String> {
    let url = format!(
        "https://{}:{}/Marti/api/tls/config",
        host.trim(),
        enroll_port.trim()
    );

    let response_text = enrollment_http_client()?
        .get(&url)
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|e| format!("failed to fetch {}: {}", url, e))?
        .text()
        .map_err(|e| format!("failed to read config response from {}: {}", url, e))?;

    let server_port = extract_tag_value(&response_text, "serverPort")
        .ok_or_else(|| "missing serverPort in /Marti/api/tls/config response".to_string())?;
    let enroll_path = extract_tag_value(&response_text, "enrollPath")
        .ok_or_else(|| "missing enrollPath in /Marti/api/tls/config response".to_string())?;

    Ok(EnrollmentConfig {
        server_port,
        enroll_path,
    })
}

fn enroll_client_certificate(
    host: &str,
    enroll_port: &str,
    enroll_path: &str,
    username: &str,
    password: &str,
    client_uid: &str,
) -> Result<EnrollmentArtifacts, String> {
    let key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256)
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
        .map_err(|e| format!("failed to generate CSR: {}", e))?
        .pem()
        .map_err(|e| format!("failed to serialize CSR to PEM: {}", e))?;

    let url = format!(
        "https://{}:{}{}?clientUid={}",
        host.trim(),
        enroll_port.trim(),
        enroll_path.trim(),
        client_uid.trim()
    );

    let response = enrollment_http_client()?
        .post(&url)
        .basic_auth(username.trim(), Some(password.to_string()))
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-pem-file")
        .body(csr)
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|e| format!("failed to enroll client certificate at {}: {}", url, e))?;

    let enrollment: EnrollmentResponse = response
        .json()
        .map_err(|e| format!("failed to parse enrollment response: {}", e))?;

    let cert_pem = wrap_pem_body(
        &enrollment.signed_cert,
        "-----BEGIN CERTIFICATE-----",
        "-----END CERTIFICATE-----",
    );
    let key_pem = key_pair.serialize_pem();

    persist_enrollment_artifacts(client_uid, &enrollment.ca0, &cert_pem, &key_pem)
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

    let enrollment_config = fetch_enrollment_config(host, enroll_port)?;
    let artifacts = enroll_client_certificate(
        host,
        enroll_port,
        &enrollment_config.enroll_path,
        username,
        password,
        &normalized_client_uid,
    )?;

    store_enrollment_artifacts(artifacts.clone());

    connect_mtls(
        &format!("{}:{}", host.trim(), enrollment_config.server_port.trim()),
        if server_name.trim().is_empty() {
            host.trim()
        } else {
            server_name.trim()
        },
        &artifacts.ca_cert_path,
        &artifacts.client_cert_path,
        &artifacts.client_key_path,
    )
}
