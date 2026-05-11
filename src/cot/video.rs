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
