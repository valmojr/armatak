use std::sync::atomic::{AtomicU8, Ordering};

static MAVLINK_SEQUENCE: AtomicU8 = AtomicU8::new(0);

#[derive(Clone, Copy)]
pub(crate) struct FieldSpec {
    pub ty: &'static str,
    pub name: &'static str,
    pub array_len: usize,
}

fn crc_accumulate(byte: u8, crc: &mut u16) {
    let mut tmp = byte ^ (*crc as u8);
    tmp ^= tmp << 4;
    *crc = (*crc >> 8) ^ ((tmp as u16) << 8) ^ ((tmp as u16) << 3) ^ ((tmp as u16) >> 4);
}

fn mavlink_crc(bytes: &[u8], crc_extra: u8) -> u16 {
    let mut crc = 0xFFFFu16;
    for byte in bytes {
        crc_accumulate(*byte, &mut crc);
    }
    crc_accumulate(crc_extra, &mut crc);
    crc
}

pub(crate) fn calculate_crc_extra(message_name: &str, base_fields: &[FieldSpec]) -> u8 {
    let mut crc = 0xFFFFu16;

    for byte in message_name.as_bytes() {
        crc_accumulate(*byte, &mut crc);
    }
    crc_accumulate(b' ', &mut crc);

    for field in base_fields {
        for byte in field.ty.as_bytes() {
            crc_accumulate(*byte, &mut crc);
        }
        crc_accumulate(b' ', &mut crc);

        for byte in field.name.as_bytes() {
            crc_accumulate(*byte, &mut crc);
        }
        crc_accumulate(b' ', &mut crc);

        if field.array_len > 0 {
            crc_accumulate(field.array_len as u8, &mut crc);
        }
    }

    ((crc & 0xFF) ^ (crc >> 8)) as u8
}

pub(crate) fn build_v1_packet(
    system_id: u8,
    component_id: u8,
    msg_id: u8,
    payload: &[u8],
    crc_extra: u8,
) -> Vec<u8> {
    let seq = MAVLINK_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut packet = Vec::with_capacity(payload.len() + 8);
    packet.push(0xFE);
    packet.push(payload.len() as u8);
    packet.push(seq);
    packet.push(system_id);
    packet.push(component_id);
    packet.push(msg_id);
    packet.extend_from_slice(payload);

    let crc = mavlink_crc(&packet[1..], crc_extra);
    packet.push((crc & 0xFF) as u8);
    packet.push((crc >> 8) as u8);
    packet
}

pub(crate) fn build_v2_packet(
    system_id: u8,
    component_id: u8,
    msg_id: u32,
    payload: &[u8],
    crc_extra: u8,
) -> Vec<u8> {
    let seq = MAVLINK_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut packet = Vec::with_capacity(payload.len() + 12);
    packet.push(0xFD);
    packet.push(payload.len() as u8);
    packet.push(0);
    packet.push(0);
    packet.push(seq);
    packet.push(system_id);
    packet.push(component_id);
    packet.push((msg_id & 0xFF) as u8);
    packet.push(((msg_id >> 8) & 0xFF) as u8);
    packet.push(((msg_id >> 16) & 0xFF) as u8);
    packet.extend_from_slice(payload);

    let crc = mavlink_crc(&packet[1..], crc_extra);
    packet.push((crc & 0xFF) as u8);
    packet.push((crc >> 8) as u8);
    packet
}

#[cfg(test)]
mod tests {
    use super::{build_v1_packet, build_v2_packet, calculate_crc_extra, mavlink_crc, FieldSpec};

    fn assert_packet_crc(packet: &[u8], crc_extra: u8) {
        let checksum_offset = packet.len() - 2;
        let expected = mavlink_crc(&packet[1..checksum_offset], crc_extra);
        let actual = u16::from_le_bytes([packet[checksum_offset], packet[checksum_offset + 1]]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn crc_extra_is_deterministic_for_scalar_and_array_fields() {
        let fields = [
            FieldSpec {
                ty: "uint32_t",
                name: "time_boot_ms",
                array_len: 0,
            },
            FieldSpec {
                ty: "char",
                name: "name",
                array_len: 16,
            },
        ];

        let first = calculate_crc_extra("ARMATAK_TEST", &fields);
        let second = calculate_crc_extra("ARMATAK_TEST", &fields);
        let no_fields = calculate_crc_extra("ARMATAK_TEST", &[]);

        assert_eq!(first, second);
        assert_ne!(first, no_fields);
    }

    #[test]
    fn builds_valid_mavlink_v1_packet() {
        let payload = [0x10, 0x20, 0x30];
        let packet = build_v1_packet(7, 9, 42, &payload, 77);

        assert_eq!(packet[0], 0xFE);
        assert_eq!(packet[1], payload.len() as u8);
        assert_eq!(packet[3], 7);
        assert_eq!(packet[4], 9);
        assert_eq!(packet[5], 42);
        assert_eq!(&packet[6..9], &payload);
        assert_eq!(packet.len(), payload.len() + 8);
        assert_packet_crc(&packet, 77);
    }

    #[test]
    fn builds_valid_mavlink_v2_packet_with_three_byte_message_id() {
        let payload = [0xAA, 0xBB];
        let packet = build_v2_packet(11, 13, 0x01_02_03, &payload, 99);

        assert_eq!(packet[0], 0xFD);
        assert_eq!(packet[1], payload.len() as u8);
        assert_eq!(packet[2], 0);
        assert_eq!(packet[3], 0);
        assert_eq!(packet[5], 11);
        assert_eq!(packet[6], 13);
        assert_eq!(&packet[7..10], &[0x03, 0x02, 0x01]);
        assert_eq!(&packet[10..12], &payload);
        assert_eq!(packet.len(), payload.len() + 12);
        assert_packet_crc(&packet, 99);
    }
}
