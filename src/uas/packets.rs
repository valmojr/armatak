use chrono::Utc;

use super::constants::{
    AUTOPILOT_COMPONENT_ID, CAMERA_CAP_FLAGS_CAPTURE_IMAGE, CAMERA_CAP_FLAGS_CAPTURE_VIDEO,
    CAMERA_CAP_FLAGS_HAS_BASIC_ZOOM, CAMERA_CAP_FLAGS_HAS_MODES, CAMERA_CAP_FLAGS_HAS_VIDEO_STREAM,
    GIMBAL_COMPONENT_ID, GIMBAL_MANAGER_CAP_FLAGS_BASIC_PITCH_YAW, MAV_AUTOPILOT_ARDUPILOTMEGA,
    MAV_AUTOPILOT_INVALID, MAV_LANDED_STATE_IN_AIR, MAV_LANDED_STATE_ON_GROUND,
    MAV_LANDED_STATE_UNDEFINED, MAV_MODE_FLAG_CUSTOM_MODE_ENABLED, MAV_MODE_FLAG_SAFETY_ARMED,
    MAV_PROTOCOL_CAPABILITY_COMMAND_INT,
    MAV_PROTOCOL_CAPABILITY_COMPONENT_IMPLEMENTS_GIMBAL_MANAGER, MAV_PROTOCOL_CAPABILITY_FTP,
    MAV_PROTOCOL_CAPABILITY_MAVLINK2, MAV_PROTOCOL_CAPABILITY_MISSION_INT,
    MAV_PROTOCOL_CAPABILITY_SET_POSITION_TARGET_GLOBAL_INT, MAV_STATE_ACTIVE, MAV_STATE_STANDBY,
    VIDEO_STREAM_ENCODING_H264, VIDEO_STREAM_STATUS_FLAGS_RUNNING, VIDEO_STREAM_TYPE_MPEG_TS,
    VIDEO_STREAM_TYPE_RTPUDP, VIDEO_STREAM_TYPE_RTSP, VIDEO_STREAM_TYPE_TCP_MPEG,
};
use super::crc::{build_v1_packet, build_v2_packet, calculate_crc_extra, FieldSpec};
use super::identity::{fixed_string, normalize_heading_deg, uid2_from_uuid, uid64_from_uuid};
use super::payload::UasTelemetryPayload;

pub(crate) fn heartbeat_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let mut msg = Vec::with_capacity(9);
    let custom_mode = if payload.flying { 5u32 } else { 0u32 };
    msg.extend_from_slice(&custom_mode.to_le_bytes());
    msg.push(payload.vehicle_type);
    msg.push(MAV_AUTOPILOT_ARDUPILOTMEGA);
    msg.push(if payload.flying {
        MAV_MODE_FLAG_CUSTOM_MODE_ENABLED | MAV_MODE_FLAG_SAFETY_ARMED
    } else {
        MAV_MODE_FLAG_CUSTOM_MODE_ENABLED
    });
    msg.push(if payload.flying {
        MAV_STATE_ACTIVE
    } else {
        MAV_STATE_STANDBY
    });
    msg.push(3);
    build_v1_packet(payload.system_id, payload.component_id, 0, &msg, 50)
}

pub(crate) fn command_ack_packet(
    system_id: u8,
    component_id: u8,
    command: u16,
    result: u8,
) -> Vec<u8> {
    let mut msg = Vec::with_capacity(3);
    msg.extend_from_slice(&command.to_le_bytes());
    msg.push(result);
    build_v2_packet(system_id, component_id, 77, &msg, 143)
}

pub(crate) fn gps_raw_int_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let time_usec = (Utc::now().timestamp_millis().max(0) as u64) * 1_000;
    let fix_type = if payload.flying { 3u8 } else { 2u8 };

    let mut msg = Vec::with_capacity(30);
    msg.extend_from_slice(&time_usec.to_le_bytes());
    msg.extend_from_slice(&((payload.lat_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((payload.lon_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((payload.alt_msl_m * 1000.0).round() as i32).to_le_bytes());
    msg.extend_from_slice(&u16::MAX.to_le_bytes());
    msg.extend_from_slice(&u16::MAX.to_le_bytes());
    msg.extend_from_slice(&u16::MAX.to_le_bytes());
    msg.extend_from_slice(&u16::MAX.to_le_bytes());
    msg.push(fix_type);
    msg.push(10);
    build_v1_packet(payload.system_id, payload.component_id, 24, &msg, 24)
}

pub(crate) fn global_position_int_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let time_boot_ms = Utc::now().timestamp_millis().max(0) as u32;
    let vx =
        (payload.groundspeed_mps * payload.heading_deg.to_radians().sin() * 100.0).round() as i16;
    let vy =
        (payload.groundspeed_mps * payload.heading_deg.to_radians().cos() * 100.0).round() as i16;
    let heading = (normalize_heading_deg(payload.heading_deg) * 100.0).round() as u16;

    let mut msg = Vec::with_capacity(28);
    msg.extend_from_slice(&time_boot_ms.to_le_bytes());
    msg.extend_from_slice(&((payload.lat_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((payload.lon_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((payload.alt_msl_m * 1000.0).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((payload.rel_alt_m * 1000.0).round() as i32).to_le_bytes());
    msg.extend_from_slice(&vx.to_le_bytes());
    msg.extend_from_slice(&vy.to_le_bytes());
    msg.extend_from_slice(&0i16.to_le_bytes());
    msg.extend_from_slice(&heading.to_le_bytes());
    build_v1_packet(payload.system_id, payload.component_id, 33, &msg, 104)
}

pub(crate) fn attitude_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
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

pub(crate) fn vfr_hud_packet(payload: &UasTelemetryPayload) -> Vec<u8> {
    let heading = normalize_heading_deg(payload.heading_deg).round() as i16;
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

pub(crate) fn system_status_packet(system_id: u8) -> Vec<u8> {
    let fields = [
        FieldSpec {
            ty: "uint32_t",
            name: "onboard_control_sensors_present",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint32_t",
            name: "onboard_control_sensors_enabled",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint32_t",
            name: "onboard_control_sensors_health",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "load",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "voltage_battery",
            array_len: 0,
        },
        FieldSpec {
            ty: "int16_t",
            name: "current_battery",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "drop_rate_comm",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "errors_comm",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "errors_count1",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "errors_count2",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "errors_count3",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "errors_count4",
            array_len: 0,
        },
        FieldSpec {
            ty: "int8_t",
            name: "battery_remaining",
            array_len: 0,
        },
    ];
    let crc_extra = calculate_crc_extra("SYS_STATUS", &fields);

    let sensors = 0x2000u32 | 0x4000u32 | 0x8000u32 | 0x20u32;
    let mut msg = Vec::with_capacity(31);
    msg.extend_from_slice(&sensors.to_le_bytes());
    msg.extend_from_slice(&sensors.to_le_bytes());
    msg.extend_from_slice(&sensors.to_le_bytes());
    msg.extend_from_slice(&500u16.to_le_bytes());
    msg.extend_from_slice(&12000u16.to_le_bytes());
    msg.extend_from_slice(&(-1i16).to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.push(100u8);

    build_v2_packet(system_id, AUTOPILOT_COMPONENT_ID, 1, &msg, crc_extra)
}

pub(crate) fn extended_sys_state_packet(system_id: u8, flying: bool) -> Vec<u8> {
    let fields = [
        FieldSpec {
            ty: "uint8_t",
            name: "vtol_state",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint8_t",
            name: "landed_state",
            array_len: 0,
        },
    ];
    let crc_extra = calculate_crc_extra("EXTENDED_SYS_STATE", &fields);

    let mut msg = Vec::with_capacity(2);
    msg.push(MAV_LANDED_STATE_UNDEFINED);
    msg.push(if flying {
        MAV_LANDED_STATE_IN_AIR
    } else {
        MAV_LANDED_STATE_ON_GROUND
    });

    build_v2_packet(system_id, AUTOPILOT_COMPONENT_ID, 245, &msg, crc_extra)
}

pub(crate) fn autopilot_version_packet(system_id: u8, entity_uuid: &str) -> Vec<u8> {
    let fields = [
        FieldSpec {
            ty: "uint64_t",
            name: "capabilities",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint32_t",
            name: "flight_sw_version",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint32_t",
            name: "middleware_sw_version",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint32_t",
            name: "os_sw_version",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint32_t",
            name: "board_version",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint8_t",
            name: "flight_custom_version",
            array_len: 8,
        },
        FieldSpec {
            ty: "uint8_t",
            name: "middleware_custom_version",
            array_len: 8,
        },
        FieldSpec {
            ty: "uint8_t",
            name: "os_custom_version",
            array_len: 8,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "vendor_id",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint16_t",
            name: "product_id",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint64_t",
            name: "uid",
            array_len: 0,
        },
    ];
    let crc_extra = calculate_crc_extra("AUTOPILOT_VERSION", &fields);

    let capabilities = MAV_PROTOCOL_CAPABILITY_MISSION_INT
        | MAV_PROTOCOL_CAPABILITY_COMMAND_INT
        | MAV_PROTOCOL_CAPABILITY_FTP
        | MAV_PROTOCOL_CAPABILITY_SET_POSITION_TARGET_GLOBAL_INT
        | MAV_PROTOCOL_CAPABILITY_MAVLINK2
        | MAV_PROTOCOL_CAPABILITY_COMPONENT_IMPLEMENTS_GIMBAL_MANAGER;
    let uid = uid64_from_uuid(entity_uuid);
    let uid2 = uid2_from_uuid(entity_uuid);

    let mut msg = Vec::with_capacity(78);
    msg.extend_from_slice(&capabilities.to_le_bytes());
    msg.extend_from_slice(&0x010100FFu32.to_le_bytes());
    msg.extend_from_slice(&0u32.to_le_bytes());
    msg.extend_from_slice(&0u32.to_le_bytes());
    msg.extend_from_slice(&0u32.to_le_bytes());
    msg.extend_from_slice(&[0u8; 8]);
    msg.extend_from_slice(&[0u8; 8]);
    msg.extend_from_slice(&[0u8; 8]);
    msg.extend_from_slice(&0x5441u16.to_le_bytes());
    msg.extend_from_slice(&0x5541u16.to_le_bytes());
    msg.extend_from_slice(&uid.to_le_bytes());
    msg.extend_from_slice(&uid2);

    build_v2_packet(system_id, AUTOPILOT_COMPONENT_ID, 148, &msg, crc_extra)
}

pub(crate) fn home_position_packet(
    system_id: u8,
    lat_deg: f64,
    lon_deg: f64,
    alt_msl_m: f32,
    heading_deg: f32,
) -> Vec<u8> {
    let yaw = normalize_heading_deg(heading_deg).to_radians();
    let q = [(yaw * 0.5).cos(), 0.0f32, 0.0f32, (yaw * 0.5).sin()];

    let mut msg = Vec::with_capacity(52);
    msg.extend_from_slice(&((lat_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((lon_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((alt_msl_m * 1000.0).round() as i32).to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    for value in q {
        msg.extend_from_slice(&value.to_le_bytes());
    }
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());

    build_v2_packet(system_id, AUTOPILOT_COMPONENT_ID, 242, &msg, 85)
}

pub(crate) fn component_heartbeat_packet(
    system_id: u8,
    component_id: u8,
    component_type: u8,
) -> Vec<u8> {
    let mut msg = Vec::with_capacity(9);
    msg.extend_from_slice(&0u32.to_le_bytes());
    msg.push(component_type);
    msg.push(MAV_AUTOPILOT_INVALID);
    msg.push(0);
    msg.push(MAV_STATE_ACTIVE);
    msg.push(3);
    build_v2_packet(system_id, component_id, 0, &msg, 50)
}

pub(crate) fn camera_information_packet_for_component(
    system_id: u8,
    component_id: u8,
    callsign: &str,
    gimbal_device_id: u8,
) -> Vec<u8> {
    let vendor = fixed_string::<32>("ArmaTAK");
    let model = fixed_string::<32>(callsign);
    let cam_definition_uri = fixed_string::<140>("");
    let flags = CAMERA_CAP_FLAGS_CAPTURE_VIDEO
        | CAMERA_CAP_FLAGS_CAPTURE_IMAGE
        | CAMERA_CAP_FLAGS_HAS_MODES
        | CAMERA_CAP_FLAGS_HAS_BASIC_ZOOM
        | CAMERA_CAP_FLAGS_HAS_VIDEO_STREAM;

    let mut msg = Vec::with_capacity(235);
    msg.extend_from_slice(&(Utc::now().timestamp_millis().max(0) as u32).to_le_bytes());
    msg.extend_from_slice(&0x010100FFu32.to_le_bytes());
    msg.extend_from_slice(&2.8f32.to_le_bytes());
    msg.extend_from_slice(&6.4f32.to_le_bytes());
    msg.extend_from_slice(&4.8f32.to_le_bytes());
    msg.extend_from_slice(&flags.to_le_bytes());
    msg.extend_from_slice(&1280u16.to_le_bytes());
    msg.extend_from_slice(&720u16.to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.extend_from_slice(&vendor);
    msg.extend_from_slice(&model);
    msg.push(0);
    msg.extend_from_slice(&cam_definition_uri);
    msg.push(gimbal_device_id);
    msg.push(0);

    build_v2_packet(system_id, component_id, 259, &msg, 92)
}

pub(crate) fn video_stream_information_packet_for_component(
    system_id: u8,
    component_id: u8,
    callsign: &str,
    video_uri: &str,
    hfov_deg: f32,
    stream_id: u8,
    stream_count: u8,
    thermal: bool,
) -> Vec<u8> {
    let stream_type = if video_uri.starts_with("rtsp://") {
        VIDEO_STREAM_TYPE_RTSP
    } else if video_uri.starts_with("rtp://") {
        VIDEO_STREAM_TYPE_RTPUDP
    } else if video_uri.starts_with("tcp://") {
        VIDEO_STREAM_TYPE_TCP_MPEG
    } else if video_uri.starts_with("mpegts://") || video_uri.starts_with("udp://") {
        VIDEO_STREAM_TYPE_MPEG_TS
    } else {
        VIDEO_STREAM_TYPE_RTSP
    };

    let name = fixed_string::<32>(callsign);
    let uri = fixed_string::<160>(video_uri);
    let flags = VIDEO_STREAM_STATUS_FLAGS_RUNNING | if thermal { 2 } else { 0 };

    let mut msg = Vec::with_capacity(208);
    msg.extend_from_slice(&30f32.to_le_bytes());
    msg.extend_from_slice(&4_000_000u32.to_le_bytes());
    msg.extend_from_slice(&flags.to_le_bytes());
    msg.extend_from_slice(&1280u16.to_le_bytes());
    msg.extend_from_slice(&720u16.to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.extend_from_slice(&(hfov_deg.clamp(1.0, 360.0).round() as u16).to_le_bytes());
    msg.push(stream_id);
    msg.push(stream_count);
    msg.push(stream_type);
    msg.extend_from_slice(&name);
    msg.extend_from_slice(&uri);
    msg.push(VIDEO_STREAM_ENCODING_H264);
    msg.push(0);

    build_v2_packet(system_id, component_id, 269, &msg, 109)
}

pub(crate) fn video_stream_status_packet_for_component(
    system_id: u8,
    component_id: u8,
    hfov_deg: f32,
    stream_id: u8,
    thermal: bool,
) -> Vec<u8> {
    let flags = VIDEO_STREAM_STATUS_FLAGS_RUNNING | if thermal { 2 } else { 0 };
    let mut msg = Vec::with_capacity(19);
    msg.extend_from_slice(&30f32.to_le_bytes());
    msg.extend_from_slice(&4_000_000u32.to_le_bytes());
    msg.extend_from_slice(&flags.to_le_bytes());
    msg.extend_from_slice(&1280u16.to_le_bytes());
    msg.extend_from_slice(&720u16.to_le_bytes());
    msg.extend_from_slice(&0u16.to_le_bytes());
    msg.extend_from_slice(&(hfov_deg.clamp(1.0, 360.0).round() as u16).to_le_bytes());
    msg.push(stream_id);
    msg.push(0);

    build_v2_packet(system_id, component_id, 270, &msg, 59)
}

pub(crate) fn mount_orientation_packet_for_component(
    system_id: u8,
    component_id: u8,
    pitch_deg: f32,
    yaw_deg: f32,
) -> Vec<u8> {
    let mut msg = Vec::with_capacity(16);
    msg.extend_from_slice(&(Utc::now().timestamp_millis().max(0) as u32).to_le_bytes());
    msg.extend_from_slice(&pitch_deg.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&normalize_heading_deg(yaw_deg).to_le_bytes());

    build_v2_packet(system_id, component_id, 265, &msg, 26)
}

pub(crate) fn camera_fov_status_packet_for_component(
    system_id: u8,
    component_id: u8,
    lat_camera_deg: f64,
    lon_camera_deg: f64,
    alt_camera_msl_m: f32,
    lat_image_deg: f64,
    lon_image_deg: f64,
    alt_image_msl_m: f32,
    roll_deg: f32,
    pitch_deg: f32,
    yaw_deg: f32,
    hfov_deg: f32,
    vfov_deg: f32,
) -> Vec<u8> {
    let q = attitude_quaternion(roll_deg, pitch_deg, yaw_deg);
    let mut msg = Vec::with_capacity(53);
    msg.extend_from_slice(&(Utc::now().timestamp_millis().max(0) as u32).to_le_bytes());
    msg.extend_from_slice(&((lat_camera_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((lon_camera_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((alt_camera_msl_m * 1000.0).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((lat_image_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((lon_image_deg * 1e7).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((alt_image_msl_m * 1000.0).round() as i32).to_le_bytes());
    for value in q {
        msg.extend_from_slice(&value.to_le_bytes());
    }
    msg.extend_from_slice(&hfov_deg.to_le_bytes());
    msg.extend_from_slice(&vfov_deg.to_le_bytes());
    msg.push(0);

    build_v2_packet(system_id, component_id, 271, &msg, 22)
}

pub(crate) fn mount_status_packet(
    system_id: u8,
    pitch_deg: f32,
    roll_deg: f32,
    relative_yaw_deg: f32,
) -> Vec<u8> {
    let mut msg = Vec::with_capacity(15);
    msg.extend_from_slice(&((pitch_deg * 100.0).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((roll_deg * 100.0).round() as i32).to_le_bytes());
    msg.extend_from_slice(&((relative_yaw_deg * 100.0).round() as i32).to_le_bytes());
    msg.push(system_id);
    msg.push(GIMBAL_COMPONENT_ID);
    msg.push(2);

    build_v2_packet(system_id, AUTOPILOT_COMPONENT_ID, 158, &msg, 134)
}

pub(crate) fn gimbal_manager_information_packet(system_id: u8) -> Vec<u8> {
    let fields = [
        FieldSpec {
            ty: "uint32_t",
            name: "time_boot_ms",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint32_t",
            name: "cap_flags",
            array_len: 0,
        },
        FieldSpec {
            ty: "float",
            name: "roll_min",
            array_len: 0,
        },
        FieldSpec {
            ty: "float",
            name: "roll_max",
            array_len: 0,
        },
        FieldSpec {
            ty: "float",
            name: "pitch_min",
            array_len: 0,
        },
        FieldSpec {
            ty: "float",
            name: "pitch_max",
            array_len: 0,
        },
        FieldSpec {
            ty: "float",
            name: "yaw_min",
            array_len: 0,
        },
        FieldSpec {
            ty: "float",
            name: "yaw_max",
            array_len: 0,
        },
        FieldSpec {
            ty: "uint8_t",
            name: "gimbal_device_id",
            array_len: 0,
        },
    ];
    let crc_extra = calculate_crc_extra("GIMBAL_MANAGER_INFORMATION", &fields);

    let mut msg = Vec::with_capacity(33);
    msg.extend_from_slice(&(Utc::now().timestamp_millis().max(0) as u32).to_le_bytes());
    msg.extend_from_slice(&GIMBAL_MANAGER_CAP_FLAGS_BASIC_PITCH_YAW.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&0f32.to_le_bytes());
    msg.extend_from_slice(&(-90f32).to_radians().to_le_bytes());
    msg.extend_from_slice(&30f32.to_radians().to_le_bytes());
    msg.extend_from_slice(&(-180f32).to_radians().to_le_bytes());
    msg.extend_from_slice(&180f32.to_radians().to_le_bytes());
    msg.push(GIMBAL_COMPONENT_ID);

    build_v2_packet(system_id, GIMBAL_COMPONENT_ID, 280, &msg, crc_extra)
}

fn attitude_quaternion(roll_deg: f32, pitch_deg: f32, yaw_deg: f32) -> [f32; 4] {
    let (roll, pitch, yaw) = (
        roll_deg.to_radians(),
        pitch_deg.to_radians(),
        normalize_heading_deg(yaw_deg).to_radians(),
    );
    let (sr, cr) = (roll * 0.5).sin_cos();
    let (sp, cp) = (pitch * 0.5).sin_cos();
    let (sy, cy) = (yaw * 0.5).sin_cos();

    [
        cr * cp * cy + sr * sp * sy,
        sr * cp * cy - cr * sp * sy,
        cr * sp * cy + sr * cp * sy,
        cr * cp * sy - sr * sp * cy,
    ]
}
