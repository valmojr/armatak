use arma_rs::{FromArma, FromArmaError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationPayload {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
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
        let (latitude, longitude, altitude, bearing) = <(f32, f32, f32, f32)>::from_arma(data)?;
        Ok(Self {
            latitude,
            longitude,
            altitude,
            bearing,
        })
    }
}
