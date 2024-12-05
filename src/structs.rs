use arma_rs::{FromArma, FromArmaError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationPayload {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub bearing: f32,
}

pub trait IntoMessage {
    fn into_message(self) -> String;
}

impl IntoMessage for String {
    fn into_message(self) -> String {
        self
    }
}

impl IntoMessage for LocationPayload {
    fn into_message(self) -> String {
        serde_json::to_string(&self).unwrap() // Convert struct to JSON
    }
}

impl FromArma for LocationPayload {
    fn from_arma(data: String) -> Result<LocationPayload, FromArmaError> {
        let (latitude, longitude, altitude, bearing) = <(f64, f64, f64, f32)>::from_arma(data)?;
        Ok(Self {
            latitude,
            longitude,
            altitude,
            bearing,
        })
    }
}


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
            api_auth_token
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
