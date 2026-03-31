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
    pub video_url: Option<String>,
}

impl FromArma for MarkerCoTPayload {
    fn from_arma(data: String) -> Result<MarkerCoTPayload, FromArmaError> {
        if let Ok((
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            track_course,
            track_speed,
            video_url,
        )) = <(String, String, f64, f64, f32, String, i32, f32, String)>::from_arma(data.clone())
        {
            return Ok(Self {
                uuid,
                r#type,
                point_lat,
                point_lon,
                point_hae,
                contact_callsign,
                track_course,
                track_speed,
                video_url: if video_url.trim().is_empty() {
                    None
                } else {
                    Some(video_url)
                },
            });
        }

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
            video_url: None,
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
            remarker: None,
            video_url: self.video_url.clone(),
        }
    }
}
