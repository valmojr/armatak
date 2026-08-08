use arma_rs::{FromArma, FromArmaError};

use super::cot::CursorOverTime;

pub struct ReportMarkerCoTPayload {
    pub uuid: String,
    pub r#type: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub contact_callsign: String,
    pub stale_seconds: i64,
    pub remarks: String,
}

impl FromArma for ReportMarkerCoTPayload {
    fn from_arma(data: String) -> Result<ReportMarkerCoTPayload, FromArmaError> {
        let (
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            stale_seconds,
            remarks,
        ) = <(String, String, f64, f64, f32, String, i64, String)>::from_arma(data)?;

        Ok(Self {
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            stale_seconds,
            remarks,
        })
    }
}

impl ReportMarkerCoTPayload {
    pub fn to_cot(&self) -> CursorOverTime {
        CursorOverTime {
            uuid: Some(self.uuid.clone()),
            r#type: Some(self.r#type.clone()),
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
            link_uid: None,
            remarker: Some(self.remarks.clone()),
            video_url: None,
            stale_seconds: Some(self.stale_seconds),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReportMarkerCoTPayload;
    use arma_rs::FromArma;

    #[test]
    fn parses_report_marker_and_maps_type_remarks_and_stale_time() {
        let payload = ReportMarkerCoTPayload::from_arma(
            r#"["report-1","b-m-p-s-p-i",-30.1,-51.2,42.0,"SPOTREP",900,"Observed vehicle"]"#
                .to_string(),
        )
        .expect("report marker should parse");

        assert_eq!(payload.uuid, "report-1");
        assert_eq!(payload.r#type, "b-m-p-s-p-i");
        assert_eq!(payload.stale_seconds, 900);
        assert_eq!(payload.remarks, "Observed vehicle");

        let cot = payload.to_cot();
        assert_eq!(cot.uuid.as_deref(), Some("report-1"));
        assert_eq!(cot.r#type.as_deref(), Some("b-m-p-s-p-i"));
        assert_eq!(cot.point_lat, -30.1);
        assert_eq!(cot.point_lon, -51.2);
        assert_eq!(cot.point_hae, 42.0);
        assert_eq!(cot.contact_callsign, "SPOTREP");
        assert_eq!(cot.remarker.as_deref(), Some("Observed vehicle"));
        assert_eq!(cot.stale_seconds, Some(900));
        assert!(cot.group_name.is_none());
        assert!(cot.track_course.is_none());
        assert!(cot.link_uid.is_none());
        assert!(cot.video_url.is_none());
    }
}
