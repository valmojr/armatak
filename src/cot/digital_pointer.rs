use super::cot::CursorOverTime;
use arma_rs::{FromArma, FromArmaError};

pub struct DigitalPointerPayload {
    pub link_uid: String,
    pub contact_callsign: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
}

impl FromArma for DigitalPointerPayload {
    fn from_arma(data: String) -> Result<DigitalPointerPayload, FromArmaError> {
        let (link_uid, contact_callsign, point_lat, point_lon, point_hae) =
            <(String, String, f64, f64, f32)>::from_arma(data)?;
        Ok(Self {
            link_uid,
            contact_callsign,
            point_lat,
            point_lon,
            point_hae,
        })
    }
}

impl DigitalPointerPayload {
    pub fn to_cot(&self) -> CursorOverTime {
        CursorOverTime {
            uuid: Some(format!("{}{}", self.link_uid.clone(), ".SPI1")),
            r#type: Some("b-m-p-s-p-i".to_string()),
            point_lat: self.point_lat,
            point_lon: self.point_lon,
            point_hae: self.point_hae,
            point_ce: None,
            point_le: None,
            contact_callsign: self.contact_callsign.clone(),
            group_name: None,
            group_role: None,
            track_course: None,
            track_speed: None,
            link_uid: Some(self.link_uid.clone()),
            remarker: None,
            video_url: None,
            stale_seconds: Some(7),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DigitalPointerPayload;
    use arma_rs::FromArma;

    #[test]
    fn parses_digital_pointer_and_maps_spi_contract() {
        let payload = DigitalPointerPayload::from_arma(
            r#"["operator-1","Falcon SPI",-30.1,-51.2,20.0]"#.to_string(),
        )
        .expect("digital pointer should parse");

        assert_eq!(payload.link_uid, "operator-1");
        assert_eq!(payload.contact_callsign, "Falcon SPI");

        let cot = payload.to_cot();
        assert_eq!(cot.uuid.as_deref(), Some("operator-1.SPI1"));
        assert_eq!(cot.r#type.as_deref(), Some("b-m-p-s-p-i"));
        assert_eq!(cot.point_lat, -30.1);
        assert_eq!(cot.point_lon, -51.2);
        assert_eq!(cot.point_hae, 20.0);
        assert_eq!(cot.contact_callsign, "Falcon SPI");
        assert_eq!(cot.link_uid.as_deref(), Some("operator-1"));
        assert_eq!(cot.stale_seconds, Some(7));
        assert!(cot.group_name.is_none());
        assert!(cot.track_course.is_none());
        assert!(cot.remarker.is_none());
        assert!(cot.video_url.is_none());
    }
}
