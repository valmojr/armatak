use super::cot::CursorOverTime;
use arma_rs::{FromArma, FromArmaError};

pub struct EudCoTPayload {
    pub uuid: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub contact_callsign: String,
    pub group_name: String,
    pub group_role: String,
    pub track_course: i32,
    pub track_speed: f32,
}

impl FromArma for EudCoTPayload {
    fn from_arma(data: String) -> Result<EudCoTPayload, FromArmaError> {
        let (
            uuid,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            group_name,
            group_role,
            track_course,
            track_speed,
        ) = <(String, f64, f64, f32, String, String, String, i32, f32)>::from_arma(data)?;
        Ok(Self {
            uuid,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            group_name,
            group_role,
            track_course,
            track_speed,
        })
    }
}

impl EudCoTPayload {
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
            group_name: Some(self.group_name.clone()),
            group_role: Some(self.group_role.clone()),
            track_course: Some(self.track_course),
            track_speed: Some(self.track_speed),
            link_uid: None,
            remarker: None,
            video_url: None,
            stale_seconds: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EudCoTPayload;
    use arma_rs::FromArma;

    #[test]
    fn parses_eud_payload_and_maps_all_tactical_fields() {
        let payload = EudCoTPayload::from_arma(
            r#"["eud-1",-30.1,-51.2,75.0,"Falcon","Cyan","Team Member",90,4.5]"#
                .to_string(),
        )
        .expect("EUD payload should parse");

        assert_eq!(payload.uuid, "eud-1");
        assert_eq!(payload.contact_callsign, "Falcon");
        assert_eq!(payload.group_name, "Cyan");
        assert_eq!(payload.group_role, "Team Member");

        let cot = payload.to_cot();
        assert_eq!(cot.uuid.as_deref(), Some("eud-1"));
        assert_eq!(cot.point_lat, -30.1);
        assert_eq!(cot.point_lon, -51.2);
        assert_eq!(cot.point_hae, 75.0);
        assert_eq!(cot.contact_callsign, "Falcon");
        assert_eq!(cot.group_name.as_deref(), Some("Cyan"));
        assert_eq!(cot.group_role.as_deref(), Some("Team Member"));
        assert_eq!(cot.track_course, Some(90));
        assert_eq!(cot.track_speed, Some(4.5));
        assert!(cot.link_uid.is_none());
        assert!(cot.remarker.is_none());
        assert!(cot.video_url.is_none());
        assert!(cot.stale_seconds.is_none());
    }
}
