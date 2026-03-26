pub enum ConnectionConfig {
    Plain {
        address: String,
    },
    Mtls {
        address: String,
        server_name: String,
        ca_cert_path: String,
        client_cert_path: String,
        client_key_path: String,
    },
    EnrollMtls {
        host: String,
        server_name: String,
        enroll_port: String,
        username: String,
        password: String,
        client_uid: String,
    },
}

impl ConnectionConfig {
    pub fn connected_message(&self) -> &'static str {
        match self {
            Self::Plain { .. } => "Connected to TCP Server",
            Self::Mtls { .. } => "Connected to TAK Server via mTLS",
            Self::EnrollMtls { .. } => "Connected to TAK Server via enrolled mTLS certificate",
        }
    }

    pub fn target(&self) -> String {
        match self {
            Self::Plain { address } | Self::Mtls { address, .. } => address.clone(),
            Self::EnrollMtls { host, .. } => host.clone(),
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Self::Plain { address } => format!("plain tcp -> {}", address),
            Self::Mtls {
                address,
                server_name,
                ca_cert_path,
                client_cert_path,
                client_key_path,
            } => format!(
                "manual mtls -> {} (server_name={}, ca={}, cert={}, key={})",
                address, server_name, ca_cert_path, client_cert_path, client_key_path
            ),
            Self::EnrollMtls {
                host,
                server_name,
                enroll_port,
                username,
                client_uid,
                ..
            } => format!(
                "enroll mtls -> host={} enroll_port={} server_name={} username={} client_uid={}",
                host, enroll_port, server_name, username, client_uid
            ),
        }
    }
}
