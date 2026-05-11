use arma_rs::{FromArma, FromArmaError};

use super::cot::CursorOverTime;

pub struct ReportMarkerCoTPayload {
    pub uuid: String,
    pub r#type: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub contact_callsign: String,
    pub stale_seconds: i64,
    pub remarks: String,
}

impl FromArma for ReportMarkerCoTPayload {
    fn from_arma(data: String) -> Result<ReportMarkerCoTPayload, FromArmaError> {
        let (
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            stale_seconds,
            remarks,
        ) = <(String, String, f64, f64, f32, String, i64, String)>::from_arma(data)?;

        Ok(Self {
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            stale_seconds,
            remarks,
        })
    }
}

impl ReportMarkerCoTPayload {
    pub fn to_cot(&self) -> CursorOverTime {
        CursorOverTime {
            uuid: Some(self.uuid.clone()),
            r#type: Some(self.r#type.clone()),
            point_lat: self.point_lat,
            point_lon: self.point_lon,
            point_hae: self.point_hae,
            point_ce: None,
            point_le: None,
            contact_callsign: self.contact_callsign.clone(),
            group_name: None,
            group_role: None,
            track_course: None,
            track_speed: None,
            link_uid: None,
            remarker: Some(self.remarks.clone()),
            video_url: None,
            stale_seconds: Some(self.stale_seconds),
        }
    }
}
