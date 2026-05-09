use super::constants::{MAV_TYPE_FIXED_WING, MAV_TYPE_HELICOPTER, MAV_TYPE_QUADROTOR};

pub(crate) fn map_vehicle_type(vehicle_type: u8) -> u8 {
    match vehicle_type {
        1 => MAV_TYPE_FIXED_WING,
        3 => MAV_TYPE_HELICOPTER,
        2 => MAV_TYPE_QUADROTOR,
        value => value,
    }
}

pub(crate) fn normalize_heading_deg(value: f32) -> f32 {
    ((value % 360.0) + 360.0) % 360.0
}

pub(crate) fn stable_system_id(entity_uuid: &str) -> u8 {
    let mut hash: u32 = 0x811C9DC5;
    for byte in entity_uuid.as_bytes() {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(0x01000193);
    }

    ((hash % 250) as u8) + 1
}

pub(crate) fn stable_mavlink_identity(callsign: &str, entity_uuid: &str) -> String {
    let mut identity = callsign.trim().to_string();

    for suffix in [" [ON]", " [OFF]"] {
        if identity.ends_with(suffix) {
            identity.truncate(identity.len() - suffix.len());
            break;
        }
    }

    if identity.is_empty() {
        entity_uuid.trim().to_string()
    } else {
        identity
    }
}

fn uuid16(entity_uuid: &str) -> [u8; 16] {
    let hex = entity_uuid.replace('-', "");
    let mut bytes = [0u8; 16];

    for index in 0..16 {
        let start = index * 2;
        if start + 2 <= hex.len() {
            if let Ok(value) = u8::from_str_radix(&hex[start..start + 2], 16) {
                bytes[index] = value;
            }
        }
    }

    bytes
}

pub(crate) fn uid64_from_uuid(entity_uuid: &str) -> u64 {
    let uuid = uuid16(entity_uuid);
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&uuid[..8]);
    u64::from_le_bytes(bytes)
}

pub(crate) fn uid2_from_uuid(entity_uuid: &str) -> [u8; 18] {
    let uuid = uuid16(entity_uuid);
    let mut uid2 = [0u8; 18];
    uid2[..16].copy_from_slice(&uuid);

    let checksum = entity_uuid
        .as_bytes()
        .iter()
        .fold(0u16, |acc, value| acc.wrapping_add(*value as u16));
    uid2[16] = (checksum & 0xFF) as u8;
    uid2[17] = (checksum >> 8) as u8;
    uid2
}

pub(crate) fn fixed_string<const N: usize>(value: &str) -> [u8; N] {
    let mut bytes = [0u8; N];
    let raw = value.as_bytes();
    let len = raw.len().min(N.saturating_sub(1));
    bytes[..len].copy_from_slice(&raw[..len]);
    bytes
}

pub(crate) fn should_send_video_stream_information(video_uri: &str) -> bool {
    let trimmed = video_uri.trim().to_ascii_lowercase();
    trimmed.starts_with("rtsp://")
        || trimmed.starts_with("rtp://")
        || trimmed.starts_with("udp://")
        || trimmed.starts_with("mpegts://")
        || trimmed.starts_with("tcp://")
}
