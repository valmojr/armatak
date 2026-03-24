use lazy_static::lazy_static;
use std::env;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Clone)]
pub struct EnrollmentArtifacts {
    pub ca_cert_path: String,
    pub client_cert_path: String,
    pub client_key_path: String,
}

lazy_static! {
    static ref ENROLLMENT_ARTIFACTS: Mutex<Option<EnrollmentArtifacts>> = Mutex::new(None);
}

fn current_artifacts_dir() -> Result<PathBuf, String> {
    let mut path = env::current_dir().map_err(|e| format!("failed to resolve cwd: {}", e))?;
    path.push(".armatak");
    path.push("session-certs");
    create_dir_all(&path)
        .map_err(|e| format!("failed to create cert dir {}: {}", path.display(), e))?;
    Ok(path)
}

pub fn persist_enrollment_artifacts(
    client_uid: &str,
    ca_pem: &str,
    cert_pem: &str,
    key_pem: &str,
) -> Result<EnrollmentArtifacts, String> {
    let mut base_dir = current_artifacts_dir()?;
    let safe_uid = client_uid
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();

    base_dir.push(safe_uid);
    create_dir_all(&base_dir).map_err(|e| {
        format!(
            "failed to create session cert dir {}: {}",
            base_dir.display(),
            e
        )
    })?;

    let ca_cert_path = base_dir.join("ca.pem");
    let client_cert_path = base_dir.join("client.pem");
    let client_key_path = base_dir.join("client.key");

    fs::write(&ca_cert_path, ca_pem).map_err(|e| {
        format!(
            "failed to persist CA cert {}: {}",
            ca_cert_path.display(),
            e
        )
    })?;
    fs::write(&client_cert_path, cert_pem).map_err(|e| {
        format!(
            "failed to persist client cert {}: {}",
            client_cert_path.display(),
            e
        )
    })?;
    fs::write(&client_key_path, key_pem).map_err(|e| {
        format!(
            "failed to persist client key {}: {}",
            client_key_path.display(),
            e
        )
    })?;

    Ok(EnrollmentArtifacts {
        ca_cert_path: ca_cert_path.to_string_lossy().to_string(),
        client_cert_path: client_cert_path.to_string_lossy().to_string(),
        client_key_path: client_key_path.to_string_lossy().to_string(),
    })
}

pub fn store_enrollment_artifacts(artifacts: EnrollmentArtifacts) {
    *ENROLLMENT_ARTIFACTS.lock().unwrap() = Some(artifacts);
}

pub fn clear_enrollment_artifacts() {
    if let Some(artifacts) = ENROLLMENT_ARTIFACTS.lock().unwrap().take() {
        for path in [
            artifacts.ca_cert_path,
            artifacts.client_cert_path,
            artifacts.client_key_path,
        ] {
            let _ = fs::remove_file(path);
        }
    }
}
