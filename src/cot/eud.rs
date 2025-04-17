use arma_rs::{FromArma, FromArmaError};

use super::cot::CursorOverTime;

pub struct EudCoTPayload {
  pub uuid: String,
  pub point_lat: f64,
  pub point_lon: f64,
  pub point_hae: f32,
  pub contact_callsign: String,
  pub group_name: String,
  pub group_role: String,
  pub track_course: i32,
  pub track_speed: f32,
}

impl FromArma for EudCoTPayload {
  fn from_arma(data: String) -> Result<EudCoTPayload, FromArmaError> {
      let (
          uuid,
          point_lat,
          point_lon,
          point_hae,
          contact_callsign,
          group_name,
          group_role,
          track_course,
          track_speed,
      ) = <(String, f64, f64, f32, String, String, String, i32, f32)>::from_arma(data)?;
      Ok(Self {
          uuid,
          point_lat,
          point_lon,
          point_hae,
          contact_callsign,
          group_name,
          group_role,
          track_course,
          track_speed,
      })
  }
}

impl EudCoTPayload {
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
          group_name: Some(self.group_name.clone()),
          group_role: Some(self.group_role.clone()),
          track_course: Some(self.track_course),
          track_speed: Some(self.track_speed),
          link_uid: None,
      }
  }
}
