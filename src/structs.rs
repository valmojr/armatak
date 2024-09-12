use serde::Serialize;

#[derive(Serialize)]
pub struct Marker {
    pub longitude: f64,
    pub latitude: f64,
    pub name: String,
    pub uid: String,
    pub r#type: String,
    pub course: i32,
    pub speed: i32,
    pub hae: i32,
    pub api_address: String,
    pub api_auth_token: String,
}

use arma_rs::FromArma;

impl FromArma for Marker {
    fn from_arma(data: String) -> Result<Self, String> {
        let (
            uid,
            latitude,
            longitude,
            speed,
            course,
            r#type,
            name,
            hae,
            api_address,
            api_auth_token,
        ) = <(
            String,
            f64,
            f64,
            i32,
            i32,
            String,
            String,
            i32,
            String,
            String,
        )>::from_arma(data)?;
        Ok(Self {
            uid,
            latitude,
            longitude,
            speed,
            course,
            r#type,
            name,
            hae,
            api_address,
            api_auth_token,
        })
    }
}
