use arma_rs::{FromArma, FromArmaError};

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

#[allow(dead_code)]
pub struct UasSystemPayload {
    pub address: String,
    pub entity_uuid: String,
    pub callsign: String,
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
    pub landed: bool,
    pub gimbal_roll_deg: f32,
    pub gimbal_pitch_deg: f32,
    pub gimbal_yaw_deg: f32,
    pub video_uri: String,
    pub hfov_deg: f32,
    pub vfov_deg: f32,
    pub image_lat_deg: f64,
    pub image_lon_deg: f64,
    pub image_alt_msl_m: f32,
    pub has_turret_camera: bool,
    pub battery_remaining_pct: i8,
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

impl FromArma for UasSystemPayload {
    fn from_arma(data: String) -> Result<Self, FromArmaError> {
        let (
            address,
            entity_uuid,
            callsign,
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
            landed,
            gimbal_roll_deg,
            gimbal_pitch_deg,
            gimbal_yaw_deg,
            video_uri,
            hfov_deg,
            vfov_deg,
            image_lat_deg,
            image_lon_deg,
            image_alt_msl_m,
            has_turret_camera,
            battery_remaining_pct,
        ) = <(
            String,
            String,
            String,
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
            i32,
            f32,
            f32,
            f32,
            String,
            f32,
            f32,
            f64,
            f64,
            f32,
            i32,
            i32,
        )>::from_arma(data)?;

        Ok(Self {
            address,
            entity_uuid,
            callsign,
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
            landed: landed != 0,
            gimbal_roll_deg,
            gimbal_pitch_deg,
            gimbal_yaw_deg,
            video_uri,
            hfov_deg,
            vfov_deg,
            image_lat_deg,
            image_lon_deg,
            image_alt_msl_m,
            has_turret_camera: has_turret_camera != 0,
            battery_remaining_pct: battery_remaining_pct.clamp(0, 100) as i8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{UasSystemPayload, UasTelemetryPayload};
    use arma_rs::FromArma;

    #[test]
    fn parses_and_clamps_uas_telemetry_payload() {
        let payload = UasTelemetryPayload::from_arma(
            r#"["127.0.0.1:14550",0,999,-1,-30.1,-51.2,100.5,25.25,361.0,12.5,1.0,2.0,3.0,1]"#
                .to_string(),
        )
        .expect("telemetry payload should parse");

        assert_eq!(payload.address, "127.0.0.1:14550");
        assert_eq!(payload.system_id, 1);
        assert_eq!(payload.component_id, 255);
        assert_eq!(payload.vehicle_type, 0);
        assert_eq!(payload.lat_deg, -30.1);
        assert_eq!(payload.lon_deg, -51.2);
        assert_eq!(payload.alt_msl_m, 100.5);
        assert_eq!(payload.rel_alt_m, 25.25);
        assert_eq!(payload.heading_deg, 361.0);
        assert_eq!(payload.groundspeed_mps, 12.5);
        assert_eq!(payload.roll_deg, 1.0);
        assert_eq!(payload.pitch_deg, 2.0);
        assert_eq!(payload.yaw_deg, 3.0);
        assert!(payload.flying);
    }

    #[test]
    fn parses_and_clamps_full_uas_system_payload() {
        let payload = UasSystemPayload::from_arma(
            r#"["10.0.0.1:14550","00112233-4455-6677-8899-aabbccddeeff","Falcon",300,-30.1,-51.2,100.5,25.25,90.0,12.5,1.0,2.0,3.0,0,1,4.0,5.0,6.0,"rtsp://video.example.test:8554/live",60.0,40.0,-30.2,-51.3,50.0,1,200]"#
                .to_string(),
        )
        .expect("system payload should parse");

        assert_eq!(payload.address, "10.0.0.1:14550");
        assert_eq!(payload.entity_uuid, "00112233-4455-6677-8899-aabbccddeeff");
        assert_eq!(payload.callsign, "Falcon");
        assert_eq!(payload.vehicle_type, 255);
        assert_eq!(payload.lat_deg, -30.1);
        assert_eq!(payload.lon_deg, -51.2);
        assert_eq!(payload.alt_msl_m, 100.5);
        assert_eq!(payload.rel_alt_m, 25.25);
        assert_eq!(payload.heading_deg, 90.0);
        assert_eq!(payload.groundspeed_mps, 12.5);
        assert_eq!(payload.roll_deg, 1.0);
        assert_eq!(payload.pitch_deg, 2.0);
        assert_eq!(payload.yaw_deg, 3.0);
        assert!(!payload.flying);
        assert!(payload.landed);
        assert_eq!(payload.gimbal_roll_deg, 4.0);
        assert_eq!(payload.gimbal_pitch_deg, 5.0);
        assert_eq!(payload.gimbal_yaw_deg, 6.0);
        assert_eq!(payload.video_uri, "rtsp://video.example.test:8554/live");
        assert_eq!(payload.hfov_deg, 60.0);
        assert_eq!(payload.vfov_deg, 40.0);
        assert_eq!(payload.image_lat_deg, -30.2);
        assert_eq!(payload.image_lon_deg, -51.3);
        assert_eq!(payload.image_alt_msl_m, 50.0);
        assert!(payload.has_turret_camera);
        assert_eq!(payload.battery_remaining_pct, 100);
    }
}
