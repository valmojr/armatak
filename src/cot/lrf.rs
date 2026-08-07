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

#[cfg(test)]
mod tests {
    use super::{LaserRangeFinderClearPayload, LaserRangeFinderPayload};
    use arma_rs::FromArma;
    use chrono::DateTime;

    #[test]
    fn parses_lrf_payload_and_serializes_normalized_message() {
        let payload = LaserRangeFinderPayload::from_arma(
            r#"["laser-1",-5,-10,2.5]"#.to_string(),
        )
        .expect("LRF payload should parse");

        assert_eq!(payload.uid, "laser-1");
        assert_eq!(payload.distance_meters, -5.0);
        assert_eq!(payload.azimuth_degrees, -10.0);
        assert_eq!(payload.elevation_degrees, 2.5);

        let message = payload.to_lrf_message();
        let fields: Vec<_> = message.split(',').collect();
        assert_eq!(fields.len(), 6);
        assert_eq!(fields[0], "1");
        assert_eq!(fields[1], "laser-1");
        assert!(DateTime::parse_from_rfc3339(fields[2]).is_ok());
        assert_eq!(fields[3], "0.00");
        assert_eq!(fields[4], "350.00");
        assert_eq!(fields[5], "2.50");
    }

    #[test]
    fn parses_clear_payload_and_serializes_range_error() {
        let payload = LaserRangeFinderClearPayload::from_arma(r#""laser-2""#.to_string())
            .expect("clear LRF payload should parse");
        assert_eq!(payload.uid, "laser-2");

        let message = payload.to_lrf_message();
        let fields: Vec<_> = message.split(',').collect();
        assert_eq!(fields.len(), 4);
        assert_eq!(fields[0], "1");
        assert_eq!(fields[1], "laser-2");
        assert!(DateTime::parse_from_rfc3339(fields[2]).is_ok());
        assert_eq!(fields[3], "RANGE_ERROR");
    }
}
