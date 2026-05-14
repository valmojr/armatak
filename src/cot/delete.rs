use arma_rs::{FromArma, FromArmaError};
use chrono::{Duration, SecondsFormat, Utc};

pub struct DeleteCoTPayload {
    pub target_uid: String,
    pub target_type: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
}

impl FromArma for DeleteCoTPayload {
    fn from_arma(data: String) -> Result<DeleteCoTPayload, FromArmaError> {
        let (target_uid, target_type, point_lat, point_lon, point_hae) =
            <(String, String, f64, f64, f32)>::from_arma(data)?;
        Ok(Self {
            target_uid,
            target_type,
            point_lat,
            point_lon,
            point_hae,
        })
    }
}

impl DeleteCoTPayload {
    pub fn to_xml(&self) -> String {
        let created_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let stale_time =
            (Utc::now() + Duration::seconds(60)).to_rfc3339_opts(SecondsFormat::Millis, true);

        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" ?><event type=\"t-x-d-d\" version=\"2.0\" how=\"m-g\" uid=\"{}.delete\" time=\"{}\" start=\"{}\" stale=\"{}\"><point ce=\"9999999\" le=\"9999999\" hae=\"{}\" lat=\"{}\" lon=\"{}\" /><detail><link uid=\"{}\" type=\"{}\" relation=\"none\" /><__forcedelete /></detail></event>",
            self.target_uid,
            created_time,
            created_time,
            stale_time,
            self.point_hae,
            self.point_lat,
            self.point_lon,
            self.target_uid,
            self.target_type
        )
    }
}
