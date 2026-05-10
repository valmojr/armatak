use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

use super::payload::UasSystemPayload;

#[derive(Clone)]
pub(crate) struct LatestUasSystem {
    pub mavlink_identity: String,
    pub callsign: String,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_msl_m: f32,
    pub rel_alt_m: f32,
    pub heading_deg: f32,
    pub fpv_pitch_deg: f32,
    pub fpv_yaw_deg: f32,
    pub gimbal_pitch_deg: f32,
    pub gimbal_yaw_deg: f32,
    pub video_uri: String,
    pub hfov_deg: f32,
    pub vfov_deg: f32,
    pub image_lat_deg: f64,
    pub image_lon_deg: f64,
    pub image_alt_msl_m: f32,
    pub has_turret_camera: bool,
    pub active_camera_component: u8,
    pub home_lat_deg: f64,
    pub home_lon_deg: f64,
    pub home_alt_msl_m: f32,
}

lazy_static! {
    static ref LATEST_UAS_SYSTEMS: Mutex<HashMap<u8, LatestUasSystem>> = Mutex::new(HashMap::new());
}

pub(crate) fn record_system(system_id: u8, mavlink_identity: &str, payload: &UasSystemPayload) {
    if let Ok(mut systems) = LATEST_UAS_SYSTEMS.lock() {
        let active_camera_component = systems
            .get(&system_id)
            .map(|system| system.active_camera_component)
            .unwrap_or(super::constants::CAMERA_COMPONENT_ID);
        let home = systems
            .get(&system_id)
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

        systems.insert(
            system_id,
            LatestUasSystem {
                mavlink_identity: mavlink_identity.to_string(),
                callsign: payload.callsign.clone(),
                lat_deg: payload.lat_deg,
                lon_deg: payload.lon_deg,
                alt_msl_m: payload.alt_msl_m,
                rel_alt_m: payload.rel_alt_m,
                heading_deg: payload.heading_deg,
                fpv_pitch_deg: payload.pitch_deg,
                fpv_yaw_deg: payload.yaw_deg,
                gimbal_pitch_deg: payload.gimbal_pitch_deg,
                gimbal_yaw_deg: payload.gimbal_yaw_deg,
                video_uri: payload.video_uri.clone(),
                hfov_deg: payload.hfov_deg,
                vfov_deg: payload.vfov_deg,
                image_lat_deg: payload.image_lat_deg,
                image_lon_deg: payload.image_lon_deg,
                image_alt_msl_m: payload.image_alt_msl_m,
                has_turret_camera: payload.has_turret_camera,
                active_camera_component,
                home_lat_deg: home.0,
                home_lon_deg: home.1,
                home_alt_msl_m: home.2,
            },
        );
    }
}

pub(crate) fn set_home(system_id: u8, lat_deg: f64, lon_deg: f64, alt_msl_m: f32) {
    if let Ok(mut systems) = LATEST_UAS_SYSTEMS.lock() {
        if let Some(system) = systems.get_mut(&system_id) {
            system.home_lat_deg = lat_deg;
            system.home_lon_deg = lon_deg;
            system.home_alt_msl_m = alt_msl_m;
        }
    }
}

pub(crate) fn latest_system(system_id: u8) -> Option<LatestUasSystem> {
    LATEST_UAS_SYSTEMS
        .lock()
        .ok()
        .and_then(|systems| systems.get(&system_id).cloned())
}

pub(crate) fn set_active_camera(system_id: u8, component_id: u8) {
    if let Ok(mut systems) = LATEST_UAS_SYSTEMS.lock() {
        if let Some(system) = systems.get_mut(&system_id) {
            system.active_camera_component = component_id;
        }
    }
}
