pub(crate) struct MavlinkCallbackEvent {
    pub function: &'static str,
    pub data: String,
}

use super::constants::{AUTOPILOT_COMPONENT_ID, CAMERA_COMPONENT_ID, TURRET_CAMERA_COMPONENT_ID};
use super::identity::should_send_video_stream_information;
use super::packets::{
    autopilot_version_packet, camera_fov_status_packet_for_component,
    camera_information_packet_for_component, command_ack_packet, gimbal_manager_information_packet,
    home_position_packet, mission_ack_packet, mission_request_int_packet,
    mount_orientation_packet_for_component, mount_status_packet,
    video_stream_information_packet_for_component, video_stream_status_packet_for_component,
};
use super::state::{latest_system, set_active_camera, set_home};
use log::info;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy)]
struct MissionUploadState {
    count: u16,
    next_seq: u16,
    mission_type: u8,
    gcs_system: u8,
    gcs_component: u8,
}

static MISSION_UPLOADS: OnceLock<Mutex<HashMap<u8, MissionUploadState>>> = OnceLock::new();

fn mission_uploads() -> &'static Mutex<HashMap<u8, MissionUploadState>> {
    MISSION_UPLOADS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn hex_preview(bytes: &[u8], max_len: usize) -> String {
    bytes
        .iter()
        .take(max_len)
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn mav_cmd_name(command_id: u16) -> &'static str {
    match command_id {
        16 => "NAV_WAYPOINT",
        17 => "NAV_LOITER_UNLIM",
        20 => "NAV_RETURN_TO_LAUNCH",
        21 => "NAV_LAND",
        22 => "NAV_TAKEOFF",
        176 => "DO_SET_MODE",
        178 => "DO_CHANGE_SPEED",
        179 => "DO_SET_HOME",
        192 => "DO_REPOSITION",
        205 => "DO_MOUNT_CONTROL",
        43000 => "GUIDED_CHANGE_SPEED",
        43001 => "GUIDED_CHANGE_ALTITUDE",
        43002 => "GUIDED_CHANGE_HEADING",
        200 => "IMAGE_START_CAPTURE",
        201 => "IMAGE_STOP_CAPTURE",
        250 => "VIDEO_START_CAPTURE",
        251 => "VIDEO_STOP_CAPTURE",
        252 => "DO_CONTROL_VIDEO",
        400 => "COMPONENT_ARM_DISARM",
        511 => "SET_MESSAGE_INTERVAL",
        512 => "REQUEST_MESSAGE",
        521 => "REQUEST_CAMERA_INFORMATION",
        2502 => "VIDEO_START_STREAMING",
        2503 => "VIDEO_STOP_STREAMING",
        2504 => "REQUEST_VIDEO_STREAM_INFORMATION",
        2505 => "REQUEST_VIDEO_STREAM_STATUS",
        _ => "UNKNOWN",
    }
}

fn read_i32(payload: &[u8], offset: usize) -> Option<i32> {
    payload
        .get(offset..offset + 4)
        .map(|bytes| i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_i16(payload: &[u8], offset: usize) -> Option<i16> {
    payload
        .get(offset..offset + 2)
        .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u16(payload: &[u8], offset: usize) -> Option<u16> {
    payload
        .get(offset..offset + 2)
        .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_f32(payload: &[u8], offset: usize) -> Option<f32> {
    payload
        .get(offset..offset + 4)
        .map(|bytes| f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn mavlink_message_detail(msg_id: u8, payload: &[u8]) -> String {
    match msg_id {
        76 if payload.len() >= 31 => {
            let command = u16::from_le_bytes([payload[28], payload[29]]);
            let target_system = payload[30];
            let target_component = payload.get(31).copied().unwrap_or(0);
            format!(
                " command={}({}) target={}:{}",
                command,
                mav_cmd_name(command),
                target_system,
                target_component
            )
        }
        75 if payload.len() >= 35 => {
            let command = u16::from_le_bytes([payload[28], payload[29]]);
            let target_system = payload[30];
            let target_component = payload[31];
            format!(
                " command={}({}) target={}:{}",
                command,
                mav_cmd_name(command),
                target_system,
                target_component
            )
        }
        _ => String::new(),
    }
}

pub(crate) fn mavlink_packet_summary(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "empty datagram".to_string();
    }

    match bytes[0] {
        0xFE if bytes.len() >= 8 => {
            let payload_len = bytes[1] as usize;
            let seq = bytes[2];
            let system_id = bytes[3];
            let component_id = bytes[4];
            let msg_id = bytes[5];
            let detail =
                mavlink_message_detail(msg_id, bytes.get(6..6 + payload_len).unwrap_or(&[]));
            format!(
                "MAVLink v1 msgid={}{} sysid={} compid={} seq={} payload_len={} preview={}",
                msg_id,
                detail,
                system_id,
                component_id,
                seq,
                payload_len,
                hex_preview(bytes, 24)
            )
        }
        0xFD if bytes.len() >= 12 => {
            let payload_len = bytes[1] as usize;
            let seq = bytes[4];
            let system_id = bytes[5];
            let component_id = bytes[6];
            let msg_id = bytes[7] as u32 | ((bytes[8] as u32) << 8) | ((bytes[9] as u32) << 16);
            let detail = mavlink_message_detail(
                msg_id as u8,
                bytes.get(10..10 + payload_len).unwrap_or(&[]),
            );
            format!(
                "MAVLink v2 msgid={}{} sysid={} compid={} seq={} payload_len={} preview={}",
                msg_id,
                detail,
                system_id,
                component_id,
                seq,
                payload_len,
                hex_preview(bytes, 24)
            )
        }
        _ => format!("Unknown UDP payload preview={}", hex_preview(bytes, 24)),
    }
}

pub(crate) fn mavlink_callback_event(bytes: &[u8], source: &str) -> Option<MavlinkCallbackEvent> {
    if bytes.len() < 8 {
        return None;
    }

    let (msg_id, system_id, component_id, payload) = match bytes[0] {
        0xFE if bytes.len() >= 8 => {
            let payload_len = bytes[1] as usize;
            (
                bytes[5] as u32,
                bytes[3],
                bytes[4],
                bytes.get(6..6 + payload_len).unwrap_or(&[]),
            )
        }
        0xFD if bytes.len() >= 12 => {
            let payload_len = bytes[1] as usize;
            (
                bytes[7] as u32 | ((bytes[8] as u32) << 8) | ((bytes[9] as u32) << 16),
                bytes[5],
                bytes[6],
                bytes.get(10..10 + payload_len).unwrap_or(&[]),
            )
        }
        _ => return None,
    };

    match msg_id {
        69 if payload.len() >= 11 => {
            let x = read_i16(payload, 0)?;
            let y = read_i16(payload, 2)?;
            let z = read_i16(payload, 4)?;
            let r = read_i16(payload, 6)?;
            let buttons = read_u16(payload, 8)?;
            let target = *payload.get(10)?;
            Some(MavlinkCallbackEvent {
                function: "MANUAL_CONTROL",
                data: format!(
                    "source={};sysid={};compid={};target={};x={};y={};z={};r={};buttons={}",
                    source, system_id, component_id, target, x, y, z, r, buttons
                ),
            })
        }
        76 if payload.len() >= 31 => {
            let command = read_u16(payload, 28)?;
            let target_system = *payload.get(30)?;
            let target_component = payload.get(31).copied().unwrap_or(0);
            let confirmation = payload.get(32).copied().unwrap_or(0);
            Some(MavlinkCallbackEvent {
                function: "COMMAND_LONG",
                data: format!(
                    "source={};sysid={};compid={};command={};command_name={};target_system={};target_component={};confirmation={};param1={:.3};param2={:.3};param3={:.3};param4={:.3};param5={:.3};param6={:.3};param7={:.3}",
                    source,
                    system_id,
                    component_id,
                    command,
                    mav_cmd_name(command),
                    target_system,
                    target_component,
                    confirmation,
                    read_f32(payload, 0).unwrap_or(0.0),
                    read_f32(payload, 4).unwrap_or(0.0),
                    read_f32(payload, 8).unwrap_or(0.0),
                    read_f32(payload, 12).unwrap_or(0.0),
                    read_f32(payload, 16).unwrap_or(0.0),
                    read_f32(payload, 20).unwrap_or(0.0),
                    read_f32(payload, 24).unwrap_or(0.0),
                ),
            })
        }
        75 if payload.len() >= 35 => {
            let command = read_u16(payload, 28)?;
            let target_system = *payload.get(30)?;
            let target_component = *payload.get(31)?;
            let frame = *payload.get(32)?;
            let current = *payload.get(33)?;
            let autocontinue = *payload.get(34)?;
            Some(MavlinkCallbackEvent {
                function: "COMMAND_INT",
                data: format!(
                    "source={};sysid={};compid={};command={};command_name={};target_system={};target_component={};frame={};current={};autocontinue={};param1={:.3};param2={:.3};param3={:.3};param4={:.3};x={};y={};z={:.3}",
                    source,
                    system_id,
                    component_id,
                    command,
                    mav_cmd_name(command),
                    target_system,
                    target_component,
                    frame,
                    current,
                    autocontinue,
                    read_f32(payload, 0).unwrap_or(0.0),
                    read_f32(payload, 4).unwrap_or(0.0),
                    read_f32(payload, 8).unwrap_or(0.0),
                    read_f32(payload, 12).unwrap_or(0.0),
                    read_i32(payload, 16).unwrap_or(0),
                    read_i32(payload, 20).unwrap_or(0),
                    read_f32(payload, 24).unwrap_or(0.0),
                ),
            })
        }
        44 if payload.len() >= 4 => {
            let count = read_u16(payload, 0)?;
            let target_system = *payload.get(2)?;
            let target_component = *payload.get(3)?;
            let mission_type = payload.get(4).copied().unwrap_or(0);
            Some(MavlinkCallbackEvent {
                function: "MISSION_COUNT",
                data: format!(
                    "source={};sysid={};compid={};target_system={};target_component={};count={};mission_type={}",
                    source, system_id, component_id, target_system, target_component, count, mission_type
                ),
            })
        }
        39 if payload.len() >= 37 => {
            let seq = read_u16(payload, 28)?;
            let command = read_u16(payload, 30)?;
            let target_system = *payload.get(32)?;
            let target_component = *payload.get(33)?;
            let frame = *payload.get(34)?;
            let current = *payload.get(35)?;
            let autocontinue = *payload.get(36)?;
            let mission_type = payload.get(37).copied().unwrap_or(0);
            Some(MavlinkCallbackEvent {
                function: "MISSION_ITEM",
                data: format!(
                    "source={};sysid={};compid={};seq={};command={};command_name={};target_system={};target_component={};frame={};current={};autocontinue={};mission_type={};param1={:.3};param2={:.3};param3={:.3};param4={:.3};lat={:.7};lon={:.7};alt={:.3}",
                    source,
                    system_id,
                    component_id,
                    seq,
                    command,
                    mav_cmd_name(command),
                    target_system,
                    target_component,
                    frame,
                    current,
                    autocontinue,
                    mission_type,
                    read_f32(payload, 0).unwrap_or(0.0),
                    read_f32(payload, 4).unwrap_or(0.0),
                    read_f32(payload, 8).unwrap_or(0.0),
                    read_f32(payload, 12).unwrap_or(0.0),
                    read_f32(payload, 16).unwrap_or(0.0),
                    read_f32(payload, 20).unwrap_or(0.0),
                    read_f32(payload, 24).unwrap_or(0.0),
                ),
            })
        }
        73 if payload.len() >= 37 => {
            let seq = read_u16(payload, 28)?;
            let command = read_u16(payload, 30)?;
            let target_system = *payload.get(32)?;
            let target_component = *payload.get(33)?;
            let frame = *payload.get(34)?;
            let current = *payload.get(35)?;
            let autocontinue = *payload.get(36)?;
            let mission_type = payload.get(37).copied().unwrap_or(0);
            Some(MavlinkCallbackEvent {
                function: "MISSION_ITEM_INT",
                data: format!(
                    "source={};sysid={};compid={};seq={};command={};command_name={};target_system={};target_component={};frame={};current={};autocontinue={};mission_type={};param1={:.3};param2={:.3};param3={:.3};param4={:.3};x={};y={};z={:.3};lat={:.7};lon={:.7};alt={:.3}",
                    source,
                    system_id,
                    component_id,
                    seq,
                    command,
                    mav_cmd_name(command),
                    target_system,
                    target_component,
                    frame,
                    current,
                    autocontinue,
                    mission_type,
                    read_f32(payload, 0).unwrap_or(0.0),
                    read_f32(payload, 4).unwrap_or(0.0),
                    read_f32(payload, 8).unwrap_or(0.0),
                    read_f32(payload, 12).unwrap_or(0.0),
                    read_i32(payload, 16).unwrap_or(0),
                    read_i32(payload, 20).unwrap_or(0),
                    read_f32(payload, 24).unwrap_or(0.0),
                    read_i32(payload, 16).unwrap_or(0) as f64 / 1e7,
                    read_i32(payload, 20).unwrap_or(0) as f64 / 1e7,
                    read_f32(payload, 24).unwrap_or(0.0),
                ),
            })
        }
        45 if payload.len() >= 2 => {
            let target_system = *payload.get(0)?;
            let target_component = *payload.get(1)?;
            let mission_type = payload.get(2).copied().unwrap_or(0);
            Some(MavlinkCallbackEvent {
                function: "MISSION_CLEAR_ALL",
                data: format!(
                    "source={};sysid={};compid={};target_system={};target_component={};mission_type={}",
                    source, system_id, component_id, target_system, target_component, mission_type
                ),
            })
        }
        41 if payload.len() >= 4 => {
            let seq = read_u16(payload, 0)?;
            let target_system = *payload.get(2)?;
            let target_component = *payload.get(3)?;
            Some(MavlinkCallbackEvent {
                function: "MISSION_SET_CURRENT",
                data: format!(
                    "source={};sysid={};compid={};target_system={};target_component={};seq={}",
                    source, system_id, component_id, target_system, target_component, seq
                ),
            })
        }
        243 if payload.len() >= 53 => {
            let lat_int = read_i32(payload, 0)?;
            let lon_int = read_i32(payload, 4)?;
            let alt_mm = read_i32(payload, 8)?;
            let target_system = *payload.get(52)?;
            Some(MavlinkCallbackEvent {
                function: "SET_HOME_POSITION",
                data: format!(
                    "source={};sysid={};compid={};target_system={};lat={:.7};lon={:.7};alt={:.3}",
                    source,
                    system_id,
                    component_id,
                    target_system,
                    lat_int as f64 / 1e7,
                    lon_int as f64 / 1e7,
                    alt_mm as f32 / 1000.0,
                ),
            })
        }
        86 if payload.len() >= 53 => {
            let lat_int = read_i32(payload, 4)?;
            let lon_int = read_i32(payload, 8)?;
            let alt = read_f32(payload, 12)?;
            let type_mask = read_u16(payload, 48)?;
            let target_system = *payload.get(50)?;
            let target_component = *payload.get(51)?;
            let coordinate_frame = *payload.get(52)?;
            Some(MavlinkCallbackEvent {
                function: "SET_POSITION_TARGET_GLOBAL_INT",
                data: format!(
                    "source={};sysid={};compid={};target_system={};target_component={};coordinate_frame={};type_mask={};lat={:.7};lon={:.7};alt={:.3};vx={:.3};vy={:.3};vz={:.3};yaw={:.3}",
                    source,
                    system_id,
                    component_id,
                    target_system,
                    target_component,
                    coordinate_frame,
                    type_mask,
                    lat_int as f64 / 1e7,
                    lon_int as f64 / 1e7,
                    alt,
                    read_f32(payload, 16).unwrap_or(0.0),
                    read_f32(payload, 20).unwrap_or(0.0),
                    read_f32(payload, 24).unwrap_or(0.0),
                    read_f32(payload, 40).unwrap_or(0.0),
                ),
            })
        }
        11 if payload.len() >= 6 => {
            let custom_mode = payload
                .get(0..4)
                .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))?;
            let target_system = *payload.get(4)?;
            let base_mode = *payload.get(5)?;
            Some(MavlinkCallbackEvent {
                function: "SET_MODE",
                data: format!(
                    "source={};sysid={};compid={};target_system={};base_mode={};custom_mode={}",
                    source, system_id, component_id, target_system, base_mode, custom_mode
                ),
            })
        }
        77 if payload.len() >= 3 => {
            let command = read_u16(payload, 0)?;
            let result = *payload.get(2)?;
            Some(MavlinkCallbackEvent {
                function: "COMMAND_ACK",
                data: format!(
                    "source={};sysid={};compid={};command={};command_name={};result={}",
                    source,
                    system_id,
                    component_id,
                    command,
                    mav_cmd_name(command),
                    result
                ),
            })
        }
        _ => None,
    }
}

pub(crate) fn mavlink_response_packets(bytes: &[u8]) -> Vec<Vec<u8>> {
    if bytes.len() < 8 {
        return Vec::new();
    }

    let (msg_id, system_id, component_id, payload) = match bytes[0] {
        0xFE if bytes.len() >= 8 => {
            let payload_len = bytes[1] as usize;
            (
                bytes[5] as u32,
                bytes[3],
                bytes[4],
                bytes.get(6..6 + payload_len).unwrap_or(&[]),
            )
        }
        0xFD if bytes.len() >= 12 => {
            let payload_len = bytes[1] as usize;
            (
                bytes[7] as u32 | ((bytes[8] as u32) << 8) | ((bytes[9] as u32) << 16),
                bytes[5],
                bytes[6],
                bytes.get(10..10 + payload_len).unwrap_or(&[]),
            )
        }
        _ => return Vec::new(),
    };

    match msg_id {
        76 if payload.len() >= 31 => command_long_response_packets(payload),
        75 if payload.len() >= 35 => command_int_response_packets(payload),
        44 if payload.len() >= 4 => {
            mission_count_response_packets(system_id, component_id, payload)
        }
        39 | 73 if payload.len() >= 37 => mission_item_response_packets(payload),
        45 if payload.len() >= 2 => mission_clear_all_response_packets(payload),
        243 if payload.len() >= 53 => set_home_position_response_packets(payload),
        _ => Vec::new(),
    }
}

fn command_long_response_packets(payload: &[u8]) -> Vec<Vec<u8>> {
    let command = match read_u16(payload, 28) {
        Some(command) => command,
        None => return Vec::new(),
    };
    let target_system = *payload.get(30).unwrap_or(&0);
    let target_component = *payload.get(31).unwrap_or(&0);
    if target_system == 0 {
        return Vec::new();
    }

    let ack_component = if target_component == 0 {
        AUTOPILOT_COMPONENT_ID
    } else {
        target_component
    };
    let mut packets = vec![command_ack_packet(target_system, ack_component, command, 0)];

    match command {
        512 => {
            let requested_message = read_f32(payload, 0).unwrap_or(0.0).round() as u32;
            packets.extend(requested_message_packets(target_system, requested_message));
        }
        179 => {
            if target_system != 0 {
                let use_current = read_f32(payload, 0).unwrap_or(0.0) >= 1.0;
                let lat = read_f32(payload, 16).unwrap_or(0.0) as f64;
                let lon = read_f32(payload, 20).unwrap_or(0.0) as f64;
                let alt = read_f32(payload, 24).unwrap_or(0.0);
                if use_current {
                    if let Some(system) = latest_system(target_system) {
                        set_home(
                            target_system,
                            system.lat_deg,
                            system.lon_deg,
                            system.alt_msl_m - system.rel_alt_m,
                        );
                    }
                } else if lat != 0.0 || lon != 0.0 {
                    set_home(target_system, lat, lon, alt);
                }
            }
        }
        521 => {
            if let Some(system) = latest_system(target_system) {
                let camera_component =
                    camera_component_for_target(target_component, system.has_turret_camera);
                set_active_camera(target_system, camera_component);
                info!(
                    "MAVLink camera selection command={} target_system={} target_component={} resolved_camera_component={}",
                    command, target_system, target_component, camera_component
                );
                packets.push(camera_information_packet_for_component(
                    target_system,
                    camera_component,
                    &camera_name(&system.callsign, camera_component, system.has_turret_camera),
                    gimbal_device_for_target(target_component, system.has_turret_camera),
                ));
            }
        }
        2502 | 2503 | 2505 => {
            if let Some(system) = latest_system(target_system) {
                let camera_component =
                    camera_component_for_target(target_component, system.has_turret_camera);
                set_active_camera(target_system, camera_component);
                info!(
                    "MAVLink camera selection command={} target_system={} target_component={} resolved_camera_component={}",
                    command, target_system, target_component, camera_component
                );
                packets.push(video_stream_status_packet_for_component(
                    target_system,
                    camera_component,
                    system.hfov_deg,
                    1,
                    false,
                ));
            }
        }
        2504 => {
            if let Some(system) = latest_system(target_system) {
                if should_send_video_stream_information(&system.video_uri) {
                    let camera_component =
                        camera_component_for_target(target_component, system.has_turret_camera);
                    set_active_camera(target_system, camera_component);
                    info!(
                        "MAVLink camera selection command={} target_system={} target_component={} resolved_camera_component={}",
                        command, target_system, target_component, camera_component
                    );
                    packets.push(video_stream_information_packet_for_component(
                        target_system,
                        camera_component,
                        &camera_name(&system.callsign, camera_component, system.has_turret_camera),
                        &system.video_uri,
                        system.hfov_deg,
                        1,
                        1,
                        false,
                    ));
                }
            }
        }
        _ => {}
    }

    packets
}

fn command_int_response_packets(payload: &[u8]) -> Vec<Vec<u8>> {
    let command = match read_u16(payload, 28) {
        Some(command) => command,
        None => return Vec::new(),
    };
    let target_system = *payload.get(30).unwrap_or(&0);
    let target_component = *payload.get(31).unwrap_or(&AUTOPILOT_COMPONENT_ID);
    if target_system == 0 {
        return Vec::new();
    }

    vec![command_ack_packet(
        target_system,
        if target_component == 0 {
            AUTOPILOT_COMPONENT_ID
        } else {
            target_component
        },
        command,
        0,
    )]
}

fn mission_count_response_packets(
    gcs_system: u8,
    gcs_component: u8,
    payload: &[u8],
) -> Vec<Vec<u8>> {
    let count = match read_u16(payload, 0) {
        Some(count) => count,
        None => return Vec::new(),
    };
    let target_system = *payload.get(2).unwrap_or(&0);
    let target_component = *payload.get(3).unwrap_or(&AUTOPILOT_COMPONENT_ID);
    let mission_type = payload.get(4).copied().unwrap_or(0);
    if target_system == 0 {
        return Vec::new();
    }

    let ack_component = if target_component == 0 {
        AUTOPILOT_COMPONENT_ID
    } else {
        target_component
    };

    let Ok(mut uploads) = mission_uploads().lock() else {
        return Vec::new();
    };

    if count == 0 {
        uploads.remove(&target_system);
        return vec![mission_ack_packet(
            target_system,
            ack_component,
            gcs_system,
            gcs_component,
            mission_type,
        )];
    }

    uploads.insert(
        target_system,
        MissionUploadState {
            count,
            next_seq: 0,
            mission_type,
            gcs_system,
            gcs_component,
        },
    );

    vec![mission_request_int_packet(
        target_system,
        ack_component,
        gcs_system,
        gcs_component,
        0,
        mission_type,
    )]
}

fn mission_item_response_packets(payload: &[u8]) -> Vec<Vec<u8>> {
    let seq = match read_u16(payload, 28) {
        Some(seq) => seq,
        None => return Vec::new(),
    };
    let target_system = *payload.get(32).unwrap_or(&0);
    let target_component = *payload.get(33).unwrap_or(&AUTOPILOT_COMPONENT_ID);
    if target_system == 0 {
        return Vec::new();
    }

    let ack_component = if target_component == 0 {
        AUTOPILOT_COMPONENT_ID
    } else {
        target_component
    };

    let Ok(mut uploads) = mission_uploads().lock() else {
        return Vec::new();
    };
    let Some(state) = uploads.get_mut(&target_system) else {
        return vec![mission_ack_packet(
            target_system,
            ack_component,
            255,
            190,
            payload.get(37).copied().unwrap_or(0),
        )];
    };

    state.next_seq = seq.saturating_add(1);
    if state.next_seq < state.count {
        vec![mission_request_int_packet(
            target_system,
            ack_component,
            state.gcs_system,
            state.gcs_component,
            state.next_seq,
            state.mission_type,
        )]
    } else {
        let ack = mission_ack_packet(
            target_system,
            ack_component,
            state.gcs_system,
            state.gcs_component,
            state.mission_type,
        );
        uploads.remove(&target_system);
        vec![ack]
    }
}

fn mission_clear_all_response_packets(payload: &[u8]) -> Vec<Vec<u8>> {
    let target_system = *payload.get(0).unwrap_or(&0);
    let target_component = *payload.get(1).unwrap_or(&AUTOPILOT_COMPONENT_ID);
    if target_system == 0 {
        return Vec::new();
    }

    if let Ok(mut uploads) = mission_uploads().lock() {
        uploads.remove(&target_system);
    }

    vec![mission_ack_packet(
        target_system,
        if target_component == 0 {
            AUTOPILOT_COMPONENT_ID
        } else {
            target_component
        },
        255,
        190,
        payload.get(2).copied().unwrap_or(0),
    )]
}

fn set_home_position_response_packets(payload: &[u8]) -> Vec<Vec<u8>> {
    let target_system = *payload.get(52).unwrap_or(&0);
    if target_system == 0 {
        return Vec::new();
    }
    let lat = read_i32(payload, 0).unwrap_or(0) as f64 / 1e7;
    let lon = read_i32(payload, 4).unwrap_or(0) as f64 / 1e7;
    let alt = read_i32(payload, 8).unwrap_or(0) as f32 / 1000.0;
    if lat != 0.0 || lon != 0.0 {
        set_home(target_system, lat, lon, alt);
    }
    Vec::new()
}

fn requested_message_packets(system_id: u8, requested_message: u32) -> Vec<Vec<u8>> {
    let Some(system) = latest_system(system_id) else {
        return Vec::new();
    };

    match requested_message {
        148 => vec![autopilot_version_packet(
            system_id,
            &system.mavlink_identity,
        )],
        242 => vec![home_position_packet(
            system_id,
            system.home_lat_deg,
            system.home_lon_deg,
            system.home_alt_msl_m,
            system.heading_deg,
        )],
        259 => {
            let mut packets = vec![camera_information_packet_for_component(
                system_id,
                CAMERA_COMPONENT_ID,
                &camera_name(
                    &system.callsign,
                    CAMERA_COMPONENT_ID,
                    system.has_turret_camera,
                ),
                0,
            )];
            if system.has_turret_camera {
                packets.push(camera_information_packet_for_component(
                    system_id,
                    TURRET_CAMERA_COMPONENT_ID,
                    &camera_name(&system.callsign, TURRET_CAMERA_COMPONENT_ID, true),
                    super::constants::GIMBAL_COMPONENT_ID,
                ));
            }
            packets
        }
        269 => {
            if should_send_video_stream_information(&system.video_uri) {
                let mut packets = vec![video_stream_information_packet_for_component(
                    system_id,
                    CAMERA_COMPONENT_ID,
                    &camera_name(
                        &system.callsign,
                        CAMERA_COMPONENT_ID,
                        system.has_turret_camera,
                    ),
                    &system.video_uri,
                    system.hfov_deg,
                    1,
                    1,
                    false,
                )];
                if system.has_turret_camera {
                    packets.push(video_stream_information_packet_for_component(
                        system_id,
                        TURRET_CAMERA_COMPONENT_ID,
                        &camera_name(&system.callsign, TURRET_CAMERA_COMPONENT_ID, true),
                        &system.video_uri,
                        system.hfov_deg,
                        1,
                        1,
                        false,
                    ));
                }
                packets
            } else {
                Vec::new()
            }
        }
        270 => {
            let mut packets = vec![video_stream_status_packet_for_component(
                system_id,
                CAMERA_COMPONENT_ID,
                system.hfov_deg,
                1,
                false,
            )];
            if system.has_turret_camera {
                packets.push(video_stream_status_packet_for_component(
                    system_id,
                    TURRET_CAMERA_COMPONENT_ID,
                    system.hfov_deg,
                    1,
                    false,
                ));
            }
            packets
        }
        265 => {
            let mut packets = vec![mount_orientation_packet_for_component(
                system_id,
                CAMERA_COMPONENT_ID,
                system.fpv_pitch_deg,
                system.fpv_yaw_deg,
            )];
            if system.has_turret_camera {
                packets.push(mount_orientation_packet_for_component(
                    system_id,
                    TURRET_CAMERA_COMPONENT_ID,
                    system.gimbal_pitch_deg,
                    system.gimbal_yaw_deg,
                ));
            }
            packets
        }
        271 => {
            let (fpv_image_lat, fpv_image_lon, fpv_image_alt) = fpv_image_point(
                system.lat_deg,
                system.lon_deg,
                system.alt_msl_m,
                system.rel_alt_m,
                system.fpv_pitch_deg,
                system.fpv_yaw_deg,
            );
            let mut packets = vec![camera_fov_status_packet_for_component(
                system_id,
                CAMERA_COMPONENT_ID,
                system.lat_deg,
                system.lon_deg,
                system.alt_msl_m,
                fpv_image_lat,
                fpv_image_lon,
                fpv_image_alt,
                0.0,
                system.fpv_pitch_deg,
                system.fpv_yaw_deg,
                system.hfov_deg,
                system.vfov_deg,
            )];
            if system.has_turret_camera {
                packets.push(camera_fov_status_packet_for_component(
                    system_id,
                    TURRET_CAMERA_COMPONENT_ID,
                    system.lat_deg,
                    system.lon_deg,
                    system.alt_msl_m,
                    system.image_lat_deg,
                    system.image_lon_deg,
                    system.image_alt_msl_m,
                    0.0,
                    system.gimbal_pitch_deg,
                    system.gimbal_yaw_deg,
                    system.hfov_deg,
                    system.vfov_deg,
                ));
            }
            packets
        }
        158 => {
            let active_component = if system.has_turret_camera {
                system.active_camera_component
            } else {
                CAMERA_COMPONENT_ID
            };
            let (pitch, roll, relative_yaw) = if active_component == TURRET_CAMERA_COMPONENT_ID {
                (
                    system.gimbal_pitch_deg,
                    0.0,
                    normalize_signed_deg(system.gimbal_yaw_deg - system.fpv_yaw_deg),
                )
            } else {
                (system.fpv_pitch_deg, 0.0, 0.0)
            };
            vec![mount_status_packet(system_id, pitch, roll, relative_yaw)]
        }
        280 => vec![gimbal_manager_information_packet(system_id)],
        _ => Vec::new(),
    }
}

fn camera_component_for_target(target_component: u8, has_turret_camera: bool) -> u8 {
    if has_turret_camera && target_component == TURRET_CAMERA_COMPONENT_ID {
        TURRET_CAMERA_COMPONENT_ID
    } else {
        CAMERA_COMPONENT_ID
    }
}

fn gimbal_device_for_target(target_component: u8, has_turret_camera: bool) -> u8 {
    if has_turret_camera && target_component == TURRET_CAMERA_COMPONENT_ID {
        super::constants::GIMBAL_COMPONENT_ID
    } else {
        0
    }
}

fn camera_name(callsign: &str, component_id: u8, has_turret_camera: bool) -> String {
    if has_turret_camera && component_id == TURRET_CAMERA_COMPONENT_ID {
        format!("{callsign} Turret")
    } else {
        format!("{callsign} FPV")
    }
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
