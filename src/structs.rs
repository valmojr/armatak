use arma_rs::{FromArma, FromArmaError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogPayload {
    pub status: String,
    pub message: String,
}

impl FromArma for LogPayload {
    fn from_arma(data: String) -> Result<LogPayload, FromArmaError> {
        let (status, message) = <(String, String)>::from_arma(data)?;
        Ok(Self {
            status,
            message
        })
    }
}
