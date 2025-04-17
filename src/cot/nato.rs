use arma_rs::{FromArma, FromArmaError};

use super::cot::CursorOverTime;

pub struct MarkerCoTPayload {
    pub uuid: String,
    pub r#type: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub contact_callsign: String,
    pub track_course: i32,
    pub track_speed: f32,
}

impl FromArma for MarkerCoTPayload {
    fn from_arma(data: String) -> Result<MarkerCoTPayload, FromArmaError> {
        let (
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            track_course,
            track_speed,
        ) = <(String, String, f64, f64, f32, String, i32, f32)>::from_arma(data)?;
        Ok(Self {
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            track_course,
            track_speed,
        })
    }
}

impl MarkerCoTPayload {
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
            track_course: Some(self.track_course),
            track_speed: Some(self.track_speed),
            link_uid: None,
        }
    }
}
