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

#[cfg(test)]
mod tests {
    use super::*;

    const UUID: &str = "00112233-4455-6677-8899-aabbccddeeff";

    #[test]
    fn maps_arma_vehicle_types_to_mavlink_types() {
        assert_eq!(map_vehicle_type(1), MAV_TYPE_FIXED_WING);
        assert_eq!(map_vehicle_type(2), MAV_TYPE_QUADROTOR);
        assert_eq!(map_vehicle_type(3), MAV_TYPE_HELICOPTER);
        assert_eq!(map_vehicle_type(99), 99);
    }

    #[test]
    fn normalizes_headings_to_zero_through_360() {
        assert_eq!(normalize_heading_deg(0.0), 0.0);
        assert_eq!(normalize_heading_deg(370.0), 10.0);
        assert_eq!(normalize_heading_deg(-10.0), 350.0);
        assert_eq!(normalize_heading_deg(720.0), 0.0);
    }

    #[test]
    fn stable_system_id_is_deterministic_and_in_mavlink_range() {
        let first = stable_system_id(UUID);
        let second = stable_system_id(UUID);

        assert_eq!(first, second);
        assert!((1..=250).contains(&first));
        assert!((1..=250).contains(&stable_system_id("")));
    }

    #[test]
    fn stable_identity_trims_runtime_status_suffixes_and_falls_back_to_uuid() {
        assert_eq!(stable_mavlink_identity(" Falcon ", UUID), "Falcon");
        assert_eq!(stable_mavlink_identity("Falcon [ON]", UUID), "Falcon");
        assert_eq!(stable_mavlink_identity("Falcon [OFF]", UUID), "Falcon");
        assert_eq!(stable_mavlink_identity("   ", UUID), UUID);
    }

    #[test]
    fn converts_uuid_to_fixed_binary_identifiers() {
        assert_eq!(
            uid64_from_uuid(UUID),
            u64::from_le_bytes([0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77])
        );

        let uid2 = uid2_from_uuid(UUID);
        assert_eq!(
            &uid2[..16],
            &[
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
                0xcc, 0xdd, 0xee, 0xff,
            ]
        );
        let checksum = UUID
            .as_bytes()
            .iter()
            .fold(0u16, |acc, value| acc.wrapping_add(*value as u16));
        assert_eq!(&uid2[16..], &checksum.to_le_bytes());
    }

    #[test]
    fn malformed_or_short_uuid_components_are_zero_filled() {
        let invalid = uuid16("gg");
        assert_eq!(invalid, [0; 16]);

        let short = uuid16("01");
        assert_eq!(short[0], 1);
        assert!(short[1..].iter().all(|byte| *byte == 0));
    }

    #[test]
    fn fixed_string_reserves_a_nul_terminator_and_handles_zero_length() {
        assert_eq!(fixed_string::<5>("abcdef"), [b'a', b'b', b'c', b'd', 0]);
        assert_eq!(fixed_string::<5>("ab"), [b'a', b'b', 0, 0, 0]);
        assert_eq!(fixed_string::<0>("ab"), [0u8; 0]);
    }

    #[test]
    fn identifies_supported_video_stream_schemes_case_insensitively() {
        for uri in [
            " RTSP://host/live ",
            "rtp://host/live",
            "udp://239.1.1.1:1234",
            "mpegts://host/live",
            "tcp://host:1234",
        ] {
            assert!(should_send_video_stream_information(uri), "{uri}");
        }

        assert!(!should_send_video_stream_information("https://host/live"));
        assert!(!should_send_video_stream_information(""));
    }
}
