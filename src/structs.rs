use arma_rs::FromArma;
use serde::Serialize;

#[derive(Serialize)]
pub struct Marker {
    pub uid: String,
    pub longitude: f64,
    pub latitude: f64,
    pub name: String,
    pub r#type: String,
    pub course: f64,
    pub speed: f64,
    pub hae: f64,
    pub api_address: String,
    pub api_auth_token: String,
}

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
            f64,
            f64,
            String,
            String,
            f64,
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

#[derive(Serialize)]
pub struct LoginPayload {
    pub address: String,
    pub username: String,
    pub password: String,
}

impl FromArma for LoginPayload {
    fn from_arma(data: String) -> Result<Self, String> {
        let (address, username, password) = <(String, String, String)>::from_arma(data)?;
        Ok(Self {
            address,
            username,
            password,
        })
    }
}
