use arma_rs::Context;
use log::info;
use std::net::UdpSocket;

use super::constants::{
    AUTOPILOT_COMPONENT_ID, CAMERA_COMPONENT_ID, GIMBAL_COMPONENT_ID, MAV_TYPE_CAMERA,
    MAV_TYPE_GIMBAL, TURRET_CAMERA_COMPONENT_ID,
};
use super::endpoint::socket_for_send;
use super::identity::{
    map_vehicle_type, should_send_video_stream_information, stable_mavlink_identity,
    stable_system_id,
};
use super::packets::{
    attitude_packet, autopilot_version_packet, camera_fov_status_packet_for_component,
    camera_information_packet_for_component, component_heartbeat_packet, extended_sys_state_packet,
    gimbal_manager_information_packet, global_position_int_packet, gps_raw_int_packet,
    heartbeat_packet, home_position_packet, mount_orientation_packet_for_component,
    mount_status_packet, system_status_packet, vfr_hud_packet,
    video_stream_information_packet_for_component, video_stream_status_packet_for_component,
};
use super::payload::{UasSystemPayload, UasTelemetryPayload};
use super::state::{latest_system, record_system};

fn sending_socket(ctx: &Context, error_prefix: &str) -> Result<UdpSocket, &'static str> {
    if let Some(socket) = socket_for_send() {
        return Ok(socket);
    }

    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => Ok(socket),
        Err(error) => {
            let _ = ctx.callback_data(
                "MAVLINK MOCK ERROR",
                "Failed to bind UDP socket",
                error.to_string(),
            );
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
            let _ = ctx.callback_data(
                "MAVLINK MOCK ERROR",
                "Failed to send MAVLink packet",
                error.to_string(),
            );
            info!(
                "MAVLink mock failed sending packet {} to {}: {}",
                index, payload.address, error
            );
            return "Failed to send MAVLink mock telemetry";
        }
    }

    info!(
        "MAVLink mock sent {} packets to {}",
        packets.len(),
        payload.address
    );
    "Sent MAVLink mock telemetry"
}

pub fn send_uas_system(ctx: Context, payload: UasSystemPayload) -> &'static str {
    let mavlink_identity = stable_mavlink_identity(&payload.callsign, &payload.entity_uuid);
    let system_id = stable_system_id(&mavlink_identity);
    let vehicle_type = map_vehicle_type(payload.vehicle_type);
    record_system(system_id, &mavlink_identity, &payload);
    let active_camera_component = latest_system(system_id)
        .map(|system| system.active_camera_component)
        .unwrap_or(CAMERA_COMPONENT_ID);
    let (home_lat_deg, home_lon_deg, home_alt_msl_m) = latest_system(system_id)
        .map(|system| {
            (
                system.home_lat_deg,
                system.home_lon_deg,
                system.home_alt_msl_m,
            )
        })
        .unwrap_or((
            payload.lat_deg,
            payload.lon_deg,
            payload.alt_msl_m - payload.rel_alt_m,
        ));

    info!(
        "MAVLink system send requested to {} entity_uuid={} mavlink_identity={} sysid={} callsign={} lat={} lon={} alt_msl={} rel_alt={} heading={} gimbal_pitch={} gimbal_yaw={} video_uri={}",
        payload.address,
        payload.entity_uuid,
        mavlink_identity,
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
    let (fpv_image_lat, fpv_image_lon, fpv_image_alt) = fpv_image_point(
        payload.lat_deg,
        payload.lon_deg,
        payload.alt_msl_m,
        payload.rel_alt_m,
        payload.pitch_deg,
        payload.yaw_deg,
    );
    let active_is_turret =
        payload.has_turret_camera && active_camera_component == TURRET_CAMERA_COMPONENT_ID;
    info!(
        "MAVLink active camera sysid={} active_component={} has_turret={} active_is_turret={}",
        system_id, active_camera_component, payload.has_turret_camera, active_is_turret
    );
    let (
        primary_pitch,
        primary_roll,
        primary_yaw,
        primary_image_lat,
        primary_image_lon,
        primary_image_alt,
    ) = if active_is_turret {
        (
            payload.gimbal_pitch_deg,
            payload.gimbal_roll_deg,
            payload.gimbal_yaw_deg,
            payload.image_lat_deg,
            payload.image_lon_deg,
            payload.image_alt_msl_m,
        )
    } else {
        (
            payload.pitch_deg,
            payload.roll_deg,
            payload.yaw_deg,
            fpv_image_lat,
            fpv_image_lon,
            fpv_image_alt,
        )
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
        system_status_packet(system_id, payload.battery_remaining_pct),
        extended_sys_state_packet(system_id, payload.landed),
        autopilot_version_packet(system_id, &mavlink_identity),
        home_position_packet(
            system_id,
            home_lat_deg,
            home_lon_deg,
            home_alt_msl_m,
            payload.heading_deg,
        ),
        component_heartbeat_packet(system_id, CAMERA_COMPONENT_ID, MAV_TYPE_CAMERA),
        component_heartbeat_packet(system_id, GIMBAL_COMPONENT_ID, MAV_TYPE_GIMBAL),
        camera_information_packet_for_component(
            system_id,
            CAMERA_COMPONENT_ID,
            &format!("{} FPV", payload.callsign),
            0,
        ),
        mount_orientation_packet_for_component(
            system_id,
            CAMERA_COMPONENT_ID,
            primary_pitch,
            primary_yaw,
        ),
        camera_fov_status_packet_for_component(
            system_id,
            CAMERA_COMPONENT_ID,
            payload.lat_deg,
            payload.lon_deg,
            payload.alt_msl_m,
            primary_image_lat,
            primary_image_lon,
            primary_image_alt,
            primary_roll,
            primary_pitch,
            primary_yaw,
            payload.hfov_deg,
            payload.vfov_deg,
        ),
        gimbal_manager_information_packet(system_id),
    ];

    if payload.has_turret_camera {
        packets.push(component_heartbeat_packet(
            system_id,
            TURRET_CAMERA_COMPONENT_ID,
            MAV_TYPE_CAMERA,
        ));
        packets.push(camera_information_packet_for_component(
            system_id,
            TURRET_CAMERA_COMPONENT_ID,
            &format!("{} Turret", payload.callsign),
            GIMBAL_COMPONENT_ID,
        ));
        packets.push(mount_orientation_packet_for_component(
            system_id,
            TURRET_CAMERA_COMPONENT_ID,
            payload.gimbal_pitch_deg,
            payload.gimbal_yaw_deg,
        ));
        packets.push(camera_fov_status_packet_for_component(
            system_id,
            TURRET_CAMERA_COMPONENT_ID,
            payload.lat_deg,
            payload.lon_deg,
            payload.alt_msl_m,
            payload.image_lat_deg,
            payload.image_lon_deg,
            payload.image_alt_msl_m,
            payload.gimbal_roll_deg,
            payload.gimbal_pitch_deg,
            payload.gimbal_yaw_deg,
            payload.hfov_deg,
            payload.vfov_deg,
        ));
    }

    let (active_pitch, active_roll, active_relative_yaw) = if active_is_turret {
        (
            payload.gimbal_pitch_deg,
            payload.gimbal_roll_deg,
            normalize_signed_deg(payload.gimbal_yaw_deg - payload.yaw_deg),
        )
    } else {
        (payload.pitch_deg, payload.roll_deg, 0.0)
    };
    packets.push(mount_status_packet(
        system_id,
        active_pitch,
        active_roll,
        active_relative_yaw,
    ));

    if should_send_video_stream_information(&payload.video_uri) {
        info!(
            "Sending VIDEO_STREAM_INFORMATION for sysid={} uri={}",
            system_id, payload.video_uri
        );
        packets.push(video_stream_information_packet_for_component(
            system_id,
            CAMERA_COMPONENT_ID,
            &format!("{} FPV", payload.callsign),
            &payload.video_uri,
            payload.hfov_deg,
            1,
            1,
            false,
        ));
        packets.push(video_stream_status_packet_for_component(
            system_id,
            CAMERA_COMPONENT_ID,
            payload.hfov_deg,
            1,
            false,
        ));

        if payload.has_turret_camera {
            packets.push(video_stream_information_packet_for_component(
                system_id,
                TURRET_CAMERA_COMPONENT_ID,
                &format!("{} Turret", payload.callsign),
                &payload.video_uri,
                payload.hfov_deg,
                1,
                1,
                false,
            ));
            packets.push(video_stream_status_packet_for_component(
                system_id,
                TURRET_CAMERA_COMPONENT_ID,
                payload.hfov_deg,
                1,
                false,
            ));
        }
    } else if !payload.video_uri.trim().is_empty() {
        info!(
            "Skipping VIDEO_STREAM_INFORMATION for sysid={} because URI is not a supported stream URI: {}",
            system_id, payload.video_uri
        );
    }

    for (index, packet) in packets.iter().enumerate() {
        if let Err(error) = socket.send_to(packet, &payload.address) {
            let _ = ctx.callback_data(
                "MAVLINK MOCK ERROR",
                "Failed to send MAVLink packet",
                error.to_string(),
            );
            info!(
                "MAVLink system failed sending packet {} to {}: {}",
                index, payload.address, error
            );
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

fn normalize_signed_deg(value: f32) -> f32 {
    let normalized = ((value % 360.0) + 360.0) % 360.0;
    if normalized > 180.0 {
        normalized - 360.0
    } else {
        normalized
    }
}

fn fpv_image_point(
    lat_deg: f64,
    lon_deg: f64,
    alt_msl_m: f32,
    rel_alt_m: f32,
    pitch_deg: f32,
    yaw_deg: f32,
) -> (f64, f64, f32) {
    let pitch_rad = pitch_deg.to_radians();
    let vertical = (-pitch_rad.sin()).max(0.01);
    let slant_m = (rel_alt_m.max(1.0) / vertical).clamp(1.0, 15_000.0);
    let ground_m = slant_m * pitch_rad.cos().abs();
    let yaw_rad = yaw_deg.to_radians();
    let north_m = ground_m * yaw_rad.cos();
    let east_m = ground_m * yaw_rad.sin();
    let lat_rad = lat_deg.to_radians();
    let meters_per_degree_lat = 111_320.0;
    let meters_per_degree_lon = (111_320.0 * lat_rad.cos().abs()).max(1.0);

    (
        lat_deg + north_m as f64 / meters_per_degree_lat,
        lon_deg + east_m as f64 / meters_per_degree_lon,
        alt_msl_m - rel_alt_m,
    )
}
