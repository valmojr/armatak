pub(crate) struct MavlinkCallbackEvent {
    pub function: &'static str,
    pub data: String,
}

pub(crate) fn hex_preview(bytes: &[u8], max_len: usize) -> String {
    bytes.iter()
        .take(max_len)
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn mav_cmd_name(command_id: u16) -> &'static str {
    match command_id {
        22 => "NAV_TAKEOFF",
        176 => "DO_SET_MODE",
        200 => "IMAGE_START_CAPTURE",
        201 => "IMAGE_STOP_CAPTURE",
        250 => "VIDEO_START_CAPTURE",
        251 => "VIDEO_STOP_CAPTURE",
        252 => "DO_CONTROL_VIDEO",
        400 => "COMPONENT_ARM_DISARM",
        521 => "REQUEST_MESSAGE",
        _ => "UNKNOWN",
    }
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
        76 if payload.len() >= 33 => {
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
            let detail = mavlink_message_detail(msg_id, bytes.get(6..6 + payload_len).unwrap_or(&[]));
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
            let detail = mavlink_message_detail(msg_id as u8, bytes.get(10..10 + payload_len).unwrap_or(&[]));
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
        76 if payload.len() >= 33 => {
            let command = read_u16(payload, 28)?;
            let target_system = *payload.get(30)?;
            let target_component = *payload.get(31)?;
            let confirmation = *payload.get(32)?;
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
                    "source={};sysid={};compid={};command={};command_name={};target_system={};target_component={};frame={};current={};autocontinue={};param1={:.3};param2={:.3};param3={:.3};param4={:.3}",
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
