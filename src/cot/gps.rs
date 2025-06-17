use arma_rs::{FromArma, FromArmaError};
use super::cot::CursorOverTime;

pub struct ExternalPositionPayload {
    pub uuid: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub contact_callsign: String,
    pub track_course: i32,
    pub track_speed: f32,
    pub remarker: String,
}

impl FromArma for ExternalPositionPayload {
  fn from_arma(data: String) -> Result<ExternalPositionPayload, FromArmaError> {
      let (
          uuid,
          point_lat,
          point_lon,
          point_hae,
          contact_callsign,
          track_course,
          track_speed,
          remarker,
      ) = <(String, f64, f64, f32, String, i32, f32, String)>::from_arma(data)?;
      Ok(Self {
          uuid,
          point_lat,
          point_lon,
          point_hae,
          contact_callsign,
          track_course,
          track_speed,
          remarker,
      })
  }
}

impl ExternalPositionPayload {
  pub fn to_cot(&self) -> CursorOverTime {
      CursorOverTime {
          uuid: Some(self.uuid.clone()),
          r#type: None,
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
          remarker: Some(self.remarker.clone()),
      }
  }
}
