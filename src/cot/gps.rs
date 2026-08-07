use super::cot::CursorOverTime;
use arma_rs::{FromArma, FromArmaError};

pub struct ExternalPositionPayload {
    pub uuid: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub contact_callsign: String,
    pub track_course: i32,
    pub track_speed: f32,
    pub remarker: String,
}

impl FromArma for ExternalPositionPayload {
    fn from_arma(data: String) -> Result<ExternalPositionPayload, FromArmaError> {
        let (
            uuid,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            track_course,
            track_speed,
            remarker,
        ) = <(String, f64, f64, f32, String, i32, f32, String)>::from_arma(data)?;
        Ok(Self {
            uuid,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            track_course,
            track_speed,
            remarker,
        })
    }
}

impl ExternalPositionPayload {
    pub fn to_cot(&self) -> CursorOverTime {
        CursorOverTime {
            uuid: Some(self.uuid.clone()),
            r#type: None,
            point_lat: self.point_lat,
            point_lon: self.point_lon,
            point_hae: self.point_hae,
            point_ce: None,
            point_le: None,
            contact_callsign: self.contact_callsign.clone(),
            group_name: None,
            group_role: None,
            track_course: Some(self.track_course),
            track_speed: Some(self.track_speed),
            link_uid: None,
            remarker: Some(self.remarker.clone()),
            video_url: None,
            stale_seconds: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExternalPositionPayload;
    use arma_rs::FromArma;

    #[test]
    fn parses_external_position_and_maps_it_to_generic_cot() {
        let payload = ExternalPositionPayload::from_arma(
            r#"["gps-1",-30.1,-51.2,75.0,"Falcon",270,11.5,"external GPS"]"#
                .to_string(),
        )
        .expect("external position should parse");

        assert_eq!(payload.uuid, "gps-1");
        assert_eq!(payload.point_lat, -30.1);
        assert_eq!(payload.point_lon, -51.2);
        assert_eq!(payload.point_hae, 75.0);
        assert_eq!(payload.contact_callsign, "Falcon");
        assert_eq!(payload.track_course, 270);
        assert_eq!(payload.track_speed, 11.5);
        assert_eq!(payload.remarker, "external GPS");

        let cot = payload.to_cot();
        assert_eq!(cot.uuid.as_deref(), Some("gps-1"));
        assert_eq!(cot.r#type, None);
        assert_eq!(cot.point_lat, -30.1);
        assert_eq!(cot.point_lon, -51.2);
        assert_eq!(cot.point_hae, 75.0);
        assert_eq!(cot.contact_callsign, "Falcon");
        assert_eq!(cot.track_course, Some(270));
        assert_eq!(cot.track_speed, Some(11.5));
        assert_eq!(cot.remarker.as_deref(), Some("external GPS"));
        assert!(cot.group_name.is_none());
        assert!(cot.group_role.is_none());
        assert!(cot.link_uid.is_none());
        assert!(cot.video_url.is_none());
        assert!(cot.stale_seconds.is_none());
    }
}
