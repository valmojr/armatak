use arma_rs::{FromArma, FromArmaError};
use super::cot::CursorOverTime;

pub struct DigitalPointerPayload {
  pub link_uid: String,
  pub contact_callsign: String,
  pub point_lat: f64,
  pub point_lon: f64,
  pub point_hae: f32,
}

impl FromArma for DigitalPointerPayload {
  fn from_arma(data: String) -> Result<DigitalPointerPayload, FromArmaError> {
      let (link_uid, contact_callsign, point_lat, point_lon, point_hae) =
          <(String, String, f64, f64, f32)>::from_arma(data)?;
      Ok(Self {
          link_uid,
          contact_callsign,
          point_lat,
          point_lon,
          point_hae,
      })
  }
}

impl DigitalPointerPayload {
  pub fn to_cot(&self) -> CursorOverTime {
      CursorOverTime {
          uuid: Some(format!("{}{}", self.link_uid.clone(), ".SPI1")),
          r#type: Some("b-m-p-s-p-i".to_string()),
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
          link_uid: Some(self.link_uid.clone()),
          remarker: None,
      }
  }
}
