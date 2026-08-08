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

#[cfg(test)]
mod tests {
    use super::ConnectionConfig;

    #[test]
    fn describes_plain_tcp_connection() {
        let config = ConnectionConfig::Plain {
            address: "127.0.0.1:8087".to_string(),
        };

        assert_eq!(config.connected_message(), "Connected to TCP Server");
        assert_eq!(config.target(), "127.0.0.1:8087");
        assert_eq!(config.describe(), "plain tcp -> 127.0.0.1:8087");
    }

    #[test]
    fn describes_manual_mtls_connection() {
        let config = ConnectionConfig::Mtls {
            address: "tak.example.test:8089".to_string(),
            server_name: "tak.example.test".to_string(),
            ca_cert_path: "ca.pem".to_string(),
            client_cert_path: "client.pem".to_string(),
            client_key_path: "client.key".to_string(),
        };

        assert_eq!(
            config.connected_message(),
            "Connected to TAK Server via mTLS"
        );
        assert_eq!(config.target(), "tak.example.test:8089");
        assert_eq!(
            config.describe(),
            "manual mtls -> tak.example.test:8089 (server_name=tak.example.test, ca=ca.pem, cert=client.pem, key=client.key)"
        );
    }

    #[test]
    fn enrollment_description_excludes_password() {
        let config = ConnectionConfig::EnrollMtls {
            host: "tak.example.test".to_string(),
            server_name: "tak.example.test".to_string(),
            enroll_port: "8446".to_string(),
            username: "operator".to_string(),
            password: "super-secret-password".to_string(),
            client_uid: "armatak-test".to_string(),
        };

        assert_eq!(
            config.connected_message(),
            "Connected to TAK Server via enrolled mTLS certificate"
        );
        assert_eq!(config.target(), "tak.example.test");

        let description = config.describe();
        assert_eq!(
            description,
            "enroll mtls -> host=tak.example.test enroll_port=8446 server_name=tak.example.test username=operator client_uid=armatak-test"
        );
        assert!(!description.contains("super-secret-password"));
    }
}
