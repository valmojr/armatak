fn escape_xml_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
}

fn parse_video_url(url: &str) -> Option<(String, String, String, String)> {
    let (protocol, rest) = url.trim().split_once("://")?;
    let (authority, path) = match rest.split_once('/') {
        Some((authority, path)) => (authority, format!("/{}", path)),
        None => (rest, String::new()),
    };
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host_port)| host_port);
    let (address, port) = host_port.rsplit_once(':')?;

    if protocol.is_empty() || address.is_empty() || port.is_empty() {
        return None;
    }

    Some((
        protocol.to_ascii_lowercase(),
        address.to_string(),
        port.to_string(),
        path,
    ))
}

pub fn video_detail_xml(video_url: &str, uid: &str, callsign: &str) -> String {
    let trimmed_url = video_url.trim();
    if trimmed_url.is_empty() {
        return "<__video></__video>".to_string();
    }

    let Some((protocol, address, port, path)) = parse_video_url(trimmed_url) else {
        return format!("<__video url=\"{}\"/>", escape_xml_attribute(trimmed_url));
    };

    format!(
        "<__video><ConnectionEntry protocol=\"{}\" path=\"{}\" address=\"{}\" port=\"{}\" uid=\"{}\" alias=\"{}\" roverPort=\"-1\" rtspReliable=\"0\" ignoreEmbeddedKLV=\"False\" networkTimeout=\"0\" bufferTime=\"-1\"/></__video>",
        escape_xml_attribute(&protocol),
        escape_xml_attribute(&path),
        escape_xml_attribute(&address),
        escape_xml_attribute(&port),
        escape_xml_attribute(uid),
        escape_xml_attribute(callsign),
    )
}

#[cfg(test)]
mod tests {
    use super::{escape_xml_attribute, parse_video_url, video_detail_xml};

    #[test]
    fn escapes_xml_attribute_characters() {
        assert_eq!(
            escape_xml_attribute("A&B\"<C>'"),
            "A&amp;B&quot;&lt;C&gt;&apos;"
        );
    }

    #[test]
    fn parses_video_url_with_credentials_and_path() {
        assert_eq!(
            parse_video_url(" RTSP://user:pass@video.example.test:8554/live/main "),
            Some((
                "rtsp".to_string(),
                "video.example.test".to_string(),
                "8554".to_string(),
                "/live/main".to_string(),
            ))
        );
    }

    #[test]
    fn parses_video_url_without_path() {
        assert_eq!(
            parse_video_url("udp://239.1.1.1:1234"),
            Some((
                "udp".to_string(),
                "239.1.1.1".to_string(),
                "1234".to_string(),
                String::new(),
            ))
        );
    }

    #[test]
    fn rejects_malformed_video_urls() {
        assert!(parse_video_url("video.example.test:8554/live").is_none());
        assert!(parse_video_url("rtsp://video.example.test/live").is_none());
        assert!(parse_video_url("://video.example.test:8554/live").is_none());
        assert!(parse_video_url("rtsp://:8554/live").is_none());
        assert!(parse_video_url("rtsp://video.example.test:/live").is_none());
    }

    #[test]
    fn emits_empty_video_detail_for_blank_url() {
        assert_eq!(video_detail_xml("   ", "uid", "callsign"), "<__video></__video>");
    }

    #[test]
    fn falls_back_to_escaped_url_for_unstructured_input() {
        assert_eq!(
            video_detail_xml("custom&stream\"", "uid", "callsign"),
            "<__video url=\"custom&amp;stream&quot;\"/>"
        );
    }

    #[test]
    fn emits_connection_entry_for_structured_video_url() {
        let xml = video_detail_xml(
            "RTSP://user:pass@video.example.test:8554/live?a=1&b=2",
            "uas<&\"'",
            "Falcon<&\"'",
        );

        assert!(xml.contains("protocol=\"rtsp\""));
        assert!(xml.contains("address=\"video.example.test\""));
        assert!(xml.contains("port=\"8554\""));
        assert!(xml.contains("path=\"/live?a=1&amp;b=2\""));
        assert!(xml.contains("uid=\"uas&lt;&amp;&quot;&apos;\""));
        assert!(xml.contains("alias=\"Falcon&lt;&amp;&quot;&apos;\""));
    }
}
