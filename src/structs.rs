use serde::Serialize;

#[derive(Serialize)]
pub struct Marker {
  pub longitude: f64,
  pub latitude: f64,
  pub name: String,
  pub uid: String,
  pub r#type: Option<String>,
  pub course: Option<i32>,
  pub azimuth: Option<i32>,
  pub speed: Option<i32>,
  pub battery: Option<i32>,
  pub fov: Option<i32>,
  pub ce: Option<i32>,
  pub hae: Option<i32>,
  pub le: Option<i32>,
}

pub struct MarkerPayload {
  pub api_address: String,
  pub api_auth_token: String,
  pub markers: [Marker],
}