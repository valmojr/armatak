pub(crate) const DEFAULT_MTLS_SERVER_PORT: &str = "8089";
pub(crate) const DEFAULT_ENROLL_PATH: &str = "/Marti/api/tls/signClient/v2";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EnrollmentConfig {
    pub server_port: String,
    pub enroll_path: String,
}

pub(crate) fn enrollment_config_url(host: &str, enroll_port: &str) -> String {
    format!(
        "https://{}:{}/Marti/api/tls/config",
        host.trim(),
        enroll_port.trim()
    )
}

pub(crate) fn enrollment_sign_url(
    host: &str,
    enroll_port: &str,
    enroll_path: &str,
    client_uid: &str,
) -> String {
    let path = normalize_enroll_path(enroll_path);
    format!(
        "https://{}:{}{}?clientUid={}",
        host.trim(),
        enroll_port.trim(),
        path,
        client_uid.trim()
    )
}

pub(crate) fn parse_enrollment_config(xml: &str) -> EnrollmentConfig {
    EnrollmentConfig {
        server_port: extract_tag_value(xml, "serverPort")
            .unwrap_or_else(|| DEFAULT_MTLS_SERVER_PORT.to_string()),
        enroll_path: extract_tag_value(xml, "enrollPath")
            .map(|path| normalize_enroll_path(&path))
            .unwrap_or_else(|| DEFAULT_ENROLL_PATH.to_string()),
    }
}

pub(crate) fn normalize_certificate_pem(certificate: &str) -> String {
    let trimmed = certificate.trim();
    if trimmed.contains("-----BEGIN CERTIFICATE-----") {
        format!("{}\n", trimmed)
    } else {
        wrap_pem_body(
            trimmed,
            "-----BEGIN CERTIFICATE-----",
            "-----END CERTIFICATE-----",
        )
    }
}

fn normalize_enroll_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{}", trimmed)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_official_tak_defaults() {
        assert_eq!(DEFAULT_MTLS_SERVER_PORT, "8089");
        assert_eq!(DEFAULT_ENROLL_PATH, "/Marti/api/tls/signClient/v2");
    }

    #[test]
    fn builds_enrollment_config_url_from_trimmed_values() {
        assert_eq!(
            enrollment_config_url(" tak.example.test ", " 8446 "),
            "https://tak.example.test:8446/Marti/api/tls/config"
        );
    }

    #[test]
    fn builds_enrollment_sign_url_and_normalizes_path() {
        assert_eq!(
            enrollment_sign_url(
                " tak.example.test ",
                " 8446 ",
                " Marti/api/tls/signClient/v2 ",
                " client-1 "
            ),
            "https://tak.example.test:8446/Marti/api/tls/signClient/v2?clientUid=client-1"
        );

        assert_eq!(
            enrollment_sign_url(
                "tak.example.test",
                "8446",
                "/custom/enroll",
                "client-2"
            ),
            "https://tak.example.test:8446/custom/enroll?clientUid=client-2"
        );
    }

    #[test]
    fn parses_explicit_tak_enrollment_config() {
        let xml = "<certificateConfig><serverPort>9443</serverPort><enrollPath>Marti/api/tls/signClient/v2</enrollPath></certificateConfig>";

        assert_eq!(
            parse_enrollment_config(xml),
            EnrollmentConfig {
                server_port: "9443".to_string(),
                enroll_path: "/Marti/api/tls/signClient/v2".to_string(),
            }
        );
    }

    #[test]
    fn falls_back_when_optional_config_values_are_missing() {
        let xml = "<certificateConfig><nameEntries /></certificateConfig>";

        assert_eq!(
            parse_enrollment_config(xml),
            EnrollmentConfig {
                server_port: DEFAULT_MTLS_SERVER_PORT.to_string(),
                enroll_path: DEFAULT_ENROLL_PATH.to_string(),
            }
        );
    }

    #[test]
    fn returns_none_when_config_closing_tag_is_missing() {
        assert_eq!(extract_tag_value("<serverPort>9443", "serverPort"), None);
    }

    #[test]
    fn trims_extracted_config_values() {
        let xml = "<certificateConfig><serverPort> 8089 </serverPort><enrollPath> /Marti/api/tls/signClient/v2 </enrollPath></certificateConfig>";

        assert_eq!(
            parse_enrollment_config(xml),
            EnrollmentConfig {
                server_port: "8089".to_string(),
                enroll_path: DEFAULT_ENROLL_PATH.to_string(),
            }
        );
    }

    #[test]
    fn normalizes_raw_certificate_body_to_pem() {
        let raw = format!("{}\r\n{}", "A".repeat(64), "B".repeat(8));
        let pem = normalize_certificate_pem(&raw);

        assert_eq!(
            pem,
            format!(
                "-----BEGIN CERTIFICATE-----\n{}\n{}\n-----END CERTIFICATE-----\n",
                "A".repeat(64),
                "B".repeat(8)
            )
        );
    }

    #[test]
    fn preserves_existing_pem_and_adds_a_single_trailing_newline() {
        let with_newline =
            "-----BEGIN CERTIFICATE-----\nQUJD\n-----END CERTIFICATE-----\n";
        let without_newline = "-----BEGIN CERTIFICATE-----\nQUJD\n-----END CERTIFICATE-----";

        assert_eq!(normalize_certificate_pem(with_newline), with_newline);
        assert_eq!(
            normalize_certificate_pem(without_newline),
            format!("{}\n", without_newline)
        );
    }
}
