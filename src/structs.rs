use arma_rs::{FromArma, FromArmaError};
use serde::{Deserialize, Serialize};

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
    pub api_auth_username: String,
    pub api_auth_password: String,
}

impl FromArma for Marker {
    fn from_arma(data: String) -> Result<Marker, FromArmaError> {
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
            api_auth_username,
            api_auth_password
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
            String,
            String
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
            api_auth_username,
            api_auth_password
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkerPayload {
    pub uid: String,
    pub longitude: f64,
    pub latitude: f64,
    pub name: String,
    pub r#type: String,
    pub course: f64,
    pub speed: f64,
    pub hae: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginPayload {
    pub address: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String
}

impl FromArma for LoginPayload {
    fn from_arma(data: String) -> Result<LoginPayload, FromArmaError> {
        let (address, username, password) = <(String, String, String)>::from_arma(data)?;
        Ok(Self {
            address,
            username,
            password,
        })
    }
}
