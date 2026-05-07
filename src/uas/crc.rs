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
