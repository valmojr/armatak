use arma_rs::Context;
use log::info;
use std::net::UdpSocket;

use super::constants::{AUTOPILOT_COMPONENT_ID, CAMERA_COMPONENT_ID, GIMBAL_COMPONENT_ID, MAV_TYPE_CAMERA, MAV_TYPE_GIMBAL};
use super::endpoint::socket_for_send;
use super::identity::{map_vehicle_type, should_send_video_stream_information, stable_system_id};
use super::packets::{
    attitude_packet, autopilot_version_packet, camera_information_packet,
    component_heartbeat_packet, extended_sys_state_packet, global_position_int_packet,
    gps_raw_int_packet, heartbeat_packet, system_status_packet, vfr_hud_packet,
    video_stream_information_packet,
};
use super::payload::{UasSystemPayload, UasTelemetryPayload};

fn sending_socket(ctx: &Context, error_prefix: &str) -> Result<UdpSocket, &'static str> {
    if let Some(socket) = socket_for_send() {
        return Ok(socket);
    }

    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => Ok(socket),
        Err(error) => {
            let _ = ctx.callback_data("MAVLINK MOCK ERROR", "Failed to bind UDP socket", error.to_string());
            info!("{} failed to bind UDP socket: {}", error_prefix, error);
            Err("Failed to bind MAVLink mock socket")
        }
    }
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

    let socket = match sending_socket(&ctx, "MAVLink mock") {
        Ok(socket) => socket,
        Err(message) => return message,
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

pub fn send_uas_system(ctx: Context, payload: UasSystemPayload) -> &'static str {
    let system_id = stable_system_id(&payload.entity_uuid);
    let vehicle_type = map_vehicle_type(payload.vehicle_type);

    info!(
        "MAVLink system send requested to {} entity_uuid={} sysid={} callsign={} lat={} lon={} alt_msl={} rel_alt={} heading={} gimbal_pitch={} gimbal_yaw={} video_uri={}",
        payload.address,
        payload.entity_uuid,
        system_id,
        payload.callsign,
        payload.lat_deg,
        payload.lon_deg,
        payload.alt_msl_m,
        payload.rel_alt_m,
        payload.heading_deg,
        payload.gimbal_pitch_deg,
        payload.gimbal_yaw_deg,
        payload.video_uri
    );

    let socket = match sending_socket(&ctx, "MAVLink system") {
        Ok(socket) => socket,
        Err(message) => return message,
    };

    let autopilot_payload = UasTelemetryPayload {
        address: payload.address.clone(),
        system_id,
        component_id: AUTOPILOT_COMPONENT_ID,
        vehicle_type,
        lat_deg: payload.lat_deg,
        lon_deg: payload.lon_deg,
        alt_msl_m: payload.alt_msl_m,
        rel_alt_m: payload.rel_alt_m,
        heading_deg: payload.heading_deg,
        groundspeed_mps: payload.groundspeed_mps,
        roll_deg: payload.roll_deg,
        pitch_deg: payload.pitch_deg,
        yaw_deg: payload.yaw_deg,
        flying: payload.flying,
    };

    let mut packets = vec![
        heartbeat_packet(&autopilot_payload),
        gps_raw_int_packet(&autopilot_payload),
        global_position_int_packet(&autopilot_payload),
        attitude_packet(&autopilot_payload),
        vfr_hud_packet(&autopilot_payload),
        system_status_packet(system_id),
        extended_sys_state_packet(system_id, payload.flying),
        autopilot_version_packet(system_id, &payload.entity_uuid),
        component_heartbeat_packet(system_id, CAMERA_COMPONENT_ID, MAV_TYPE_CAMERA),
        component_heartbeat_packet(system_id, GIMBAL_COMPONENT_ID, MAV_TYPE_GIMBAL),
        camera_information_packet(system_id, &payload.callsign),
    ];

    if should_send_video_stream_information(&payload.video_uri) {
        info!(
            "Sending VIDEO_STREAM_INFORMATION for sysid={} uri={}",
            system_id,
            payload.video_uri
        );
        packets.push(video_stream_information_packet(
            system_id,
            &payload.callsign,
            &payload.video_uri,
            payload.hfov_deg,
        ));
    } else if !payload.video_uri.trim().is_empty() {
        info!(
            "Skipping VIDEO_STREAM_INFORMATION for sysid={} because URI is not a supported stream URI: {}",
            system_id, payload.video_uri
        );
    }

    for (index, packet) in packets.iter().enumerate() {
        if let Err(error) = socket.send_to(packet, &payload.address) {
            let _ = ctx.callback_data("MAVLINK MOCK ERROR", "Failed to send MAVLink packet", error.to_string());
            info!("MAVLink system failed sending packet {} to {}: {}", index, payload.address, error);
            return "Failed to send MAVLink system telemetry";
        }
    }

    info!(
        "MAVLink system sent {} packets to {} for sysid={} (camera comp={}, gimbal comp={})",
        packets.len(),
        payload.address,
        system_id,
        CAMERA_COMPONENT_ID,
        GIMBAL_COMPONENT_ID
    );
    "Sent MAVLink system telemetry"
}
