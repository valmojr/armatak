use arma_rs::{Context, FromArma, FromArmaError};
use chrono::Utc;
use log::info;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicU8, Ordering};

static MAVLINK_SEQUENCE: AtomicU8 = AtomicU8::new(0);

pub struct UasTelemetryPayload {
    pub address: String,
    pub system_id: u8,
    pub component_id: u8,
    pub vehicle_type: u8,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_msl_m: f32,
    pub rel_alt_m: f32,
    pub heading_deg: f32,
    pub groundspeed_mps: f32,
    pub roll_deg: f32,
    pub pitch_deg: f32,
    pub yaw_deg: f32,
    pub flying: bool,
}

impl FromArma for UasTelemetryPayload {
    fn from_arma(data: String) -> Result<Self, FromArmaError> {
        let (
            address,
            system_id,
            component_id,
            vehicle_type,
            lat_deg,
            lon_deg,
            alt_msl_m,
            rel_alt_m,
            heading_deg,
            groundspeed_mps,
            roll_deg,
            pitch_deg,
            yaw_deg,
            flying,
        ) = <(
            String,
            i32,
            i32,
            i32,
            f64,
            f64,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            i32,
        )>::from_arma(data)?;

        Ok(Self {
            address,
            system_id: system_id.clamp(1, 255) as u8,
            component_id: component_id.clamp(1, 255) as u8,
            vehicle_type: vehicle_type.clamp(0, 255) as u8,
            lat_deg,
            lon_deg,
            alt_msl_m,
            rel_alt_m,
            heading_deg,
            groundspeed_mps,
            roll_deg,
            pitch_deg,
            yaw_deg,
            flying: flying != 0,
        })
    }
}

fn crc_accumulate(byte: u8, crc: &mut u16) {
    let mut tmp = byte ^ (*crc as u8);
    tmp ^= tmp << 4;
    *crc = (*crc >> 8) ^ ((tmp as u16) << 8) ^ ((tmp as u16) << 3) ^ ((tmp as u16) >> 4);
}

fn mavlink_crc(header_and_payload: &[u8], crc_extra: u8) -> u16 {
    let mut crc = 0xFFFFu16;
    for byte in header_and_payload {
        crc_accumulate(*byte, &mut crc);
    }
    crc_accumulate(crc_extra, &mut crc);
    crc
}

fn build_v1_packet(system_id: u8, component_id: u8, msg_id: u8, payload: &[u8], crc_extra: u8) -> Vec<u8> {
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

fn heartbeat_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let mut msg = Vec::with_capacity(9);
    msg.extend_from_slice(&0u32.to_le_bytes());
    msg.push(payload.vehicle_type);
    msg.push(0);
    msg.push(if payload.flying { 0x81 } else { 0x01 });
    msg.push(if payload.flying { 4 } else { 3 });
    msg.push(3);
    build_v1_packet(payload.system_id, payload.component_id, 0, &msg, 50)
}

fn gps_raw_int_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let now_us = Utc::now().timestamp_micros().max(0) as u64;
    let lat = (payload.lat_deg * 1e7).round() as i32;
    let lon = (payload.lon_deg * 1e7).round() as i32;
    let alt = (payload.alt_msl_m * 1000.0).round() as i32;
    let speed_cms = (payload.groundspeed_mps.max(0.0) * 100.0).round() as u16;
    let cog_cdeg = (((payload.heading_deg % 360.0 + 360.0) % 360.0) * 100.0).round() as u16;

    let mut msg = Vec::with_capacity(30);
    msg.extend_from_slice(&now_us.to_le_bytes());
    msg.extend_from_slice(&lat.to_le_bytes());
    msg.extend_from_slice(&lon.to_le_bytes());
    msg.extend_from_slice(&alt.to_le_bytes());
    msg.extend_from_slice(&100u16.to_le_bytes());
    msg.extend_from_slice(&100u16.to_le_bytes());
    msg.extend_from_slice(&speed_cms.to_le_bytes());
    msg.extend_from_slice(&cog_cdeg.to_le_bytes());
    msg.push(3);
    msg.push(10);
    build_v1_packet(payload.system_id, payload.component_id, 24, &msg, 24)
}

fn global_position_int_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let now_ms = Utc::now().timestamp_millis().max(0) as u32;
    let lat = (payload.lat_deg * 1e7).round() as i32;
    let lon = (payload.lon_deg * 1e7).round() as i32;
    let alt = (payload.alt_msl_m * 1000.0).round() as i32;
    let rel_alt = (payload.rel_alt_m.max(0.0) * 1000.0).round() as i32;
    let speed_cms = (payload.groundspeed_mps.max(0.0) * 100.0).round() as i16;
    let hdg_cdeg = (((payload.heading_deg % 360.0 + 360.0) % 360.0) * 100.0).round() as u16;

    let mut msg = Vec::with_capacity(28);
    msg.extend_from_slice(&now_ms.to_le_bytes());
    msg.extend_from_slice(&lat.to_le_bytes());
    msg.extend_from_slice(&lon.to_le_bytes());
    msg.extend_from_slice(&alt.to_le_bytes());
    msg.extend_from_slice(&rel_alt.to_le_bytes());
    msg.extend_from_slice(&speed_cms.to_le_bytes());
    msg.extend_from_slice(&0i16.to_le_bytes());
    msg.extend_from_slice(&0i16.to_le_bytes());
    msg.extend_from_slice(&hdg_cdeg.to_le_bytes());
    build_v1_packet(payload.system_id, payload.component_id, 33, &msg, 104)
}

fn attitude_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let now_ms = Utc::now().timestamp_millis().max(0) as u32;
    let roll = payload.roll_deg.to_radians();
    let pitch = payload.pitch_deg.to_radians();
    let yaw = payload.yaw_deg.to_radians();

    let mut msg = Vec::with_capacity(28);
    msg.extend_from_slice(&now_ms.to_le_bytes());
    msg.extend_from_slice(&roll.to_le_bytes());
    msg.extend_from_slice(&pitch.to_le_bytes());
    msg.extend_from_slice(&yaw.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    build_v1_packet(payload.system_id, payload.component_id, 30, &msg, 39)
}

fn vfr_hud_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let heading = (((payload.heading_deg % 360.0 + 360.0) % 360.0).round()) as i16;
    let throttle = if payload.flying { 50u16 } else { 0u16 };

    let mut msg = Vec::with_capacity(20);
    msg.extend_from_slice(&payload.groundspeed_mps.to_le_bytes());
    msg.extend_from_slice(&payload.groundspeed_mps.to_le_bytes());
    msg.extend_from_slice(&payload.alt_msl_m.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&heading.to_le_bytes());
    msg.extend_from_slice(&throttle.to_le_bytes());
    build_v1_packet(payload.system_id, payload.component_id, 74, &msg, 20)
}

pub fn send_uas_telemetry(ctx: Context, payload: UasTelemetryPayload) -> &'static str {
    info!(
        "MAVLink mock send requested to {} sysid={} compid={} lat={} lon={} alt_msl={} rel_alt={} heading={} speed={} flying={}",
        payload.address,
        payload.system_id,
        payload.component_id,
        payload.lat_deg,
        payload.lon_deg,
        payload.alt_msl_m,
        payload.rel_alt_m,
        payload.heading_deg,
        payload.groundspeed_mps,
        payload.flying
    );

    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(error) => {
            let _ = ctx.callback_data("MAVLINK MOCK ERROR", "Failed to bind UDP socket", error.to_string());
            info!("MAVLink mock failed to bind UDP socket: {}", error);
            return "Failed to bind MAVLink mock socket";
        }
    };

    let packets = [
        heartbeat_packet(&payload),
        gps_raw_int_packet(&payload),
        global_position_int_packet(&payload),
        attitude_packet(&payload),
        vfr_hud_packet(&payload),
    ];

    for (index, packet) in packets.iter().enumerate() {
        if let Err(error) = socket.send_to(packet, &payload.address) {
            let _ = ctx.callback_data("MAVLINK MOCK ERROR", "Failed to send MAVLink packet", error.to_string());
            info!("MAVLink mock failed sending packet {} to {}: {}", index, payload.address, error);
            return "Failed to send MAVLink mock telemetry";
        }
    }

    info!("MAVLink mock sent {} packets to {}", packets.len(), payload.address);
    "Sent MAVLink mock telemetry"
}
