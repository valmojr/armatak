use arma_rs::{FromArma, FromArmaError};
use chrono::{SecondsFormat, Utc};

pub struct LaserRangeFinderPayload {
    pub uid: String,
    pub distance_meters: f64,
    pub azimuth_degrees: f64,
    pub elevation_degrees: f64,
}

impl FromArma for LaserRangeFinderPayload {
    fn from_arma(data: String) -> Result<LaserRangeFinderPayload, FromArmaError> {
        let (uid, distance_meters, azimuth_degrees, elevation_degrees) =
            <(String, f64, f64, f64)>::from_arma(data)?;
        Ok(Self {
            uid,
            distance_meters,
            azimuth_degrees,
            elevation_degrees,
        })
    }
}

impl LaserRangeFinderPayload {
    pub fn to_lrf_message(&self) -> String {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        format!(
            "1,{},{},{:.2},{:.2},{:.2}",
            self.uid,
            timestamp,
            self.distance_meters.max(0.0),
            normalize_degrees(self.azimuth_degrees),
            self.elevation_degrees
        )
    }
}

pub struct LaserRangeFinderClearPayload {
    pub uid: String,
}

impl FromArma for LaserRangeFinderClearPayload {
    fn from_arma(data: String) -> Result<LaserRangeFinderClearPayload, FromArmaError> {
        let uid = String::from_arma(data)?;
        Ok(Self { uid })
    }
}

impl LaserRangeFinderClearPayload {
    pub fn to_lrf_message(&self) -> String {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        format!("1,{},{},RANGE_ERROR", self.uid, timestamp)
    }
}

fn normalize_degrees(degrees: f64) -> f64 {
    degrees.rem_euclid(360.0)
}
