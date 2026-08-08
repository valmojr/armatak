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

#[cfg(test)]
mod tests {
    use super::DeleteCoTPayload;
    use arma_rs::FromArma;

    #[test]
    fn parses_and_serializes_forced_delete_cot() {
        let payload = DeleteCoTPayload::from_arma(
            r#"["target-1","a-f-G-U-C",-30.1,-51.2,123.5]"#.to_string(),
        )
        .expect("delete CoT payload should parse");

        assert_eq!(payload.target_uid, "target-1");
        assert_eq!(payload.target_type, "a-f-G-U-C");
        assert_eq!(payload.point_lat, -30.1);
        assert_eq!(payload.point_lon, -51.2);
        assert_eq!(payload.point_hae, 123.5);

        let xml = payload.to_xml();
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>"));
        assert!(xml.contains("type=\"t-x-d-d\""));
        assert!(xml.contains("uid=\"target-1.delete\""));
        assert!(xml.contains("hae=\"123.5\" lat=\"-30.1\" lon=\"-51.2\""));
        assert!(xml.contains("<link uid=\"target-1\" type=\"a-f-G-U-C\" relation=\"none\" />"));
        assert!(xml.contains("<__forcedelete />"));
        assert!(xml.ends_with("</detail></event>"));
    }
}
