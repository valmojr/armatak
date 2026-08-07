use arma_rs::{FromArma, FromArmaError};

use super::cot::CursorOverTime;

pub struct MarkerCoTPayload {
    pub uuid: String,
    pub r#type: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub contact_callsign: String,
    pub track_course: i32,
    pub track_speed: f32,
    pub video_url: Option<String>,
}

impl FromArma for MarkerCoTPayload {
    fn from_arma(data: String) -> Result<MarkerCoTPayload, FromArmaError> {
        if let Ok((
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            track_course,
            track_speed,
            video_url,
        )) = <(String, String, f64, f64, f32, String, i32, f32, String)>::from_arma(data.clone())
        {
            return Ok(Self {
                uuid,
                r#type,
                point_lat,
                point_lon,
                point_hae,
                contact_callsign,
                track_course,
                track_speed,
                video_url: if video_url.trim().is_empty() {
                    None
                } else {
                    Some(video_url)
                },
            });
        }

        let (
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            track_course,
            track_speed,
        ) = <(String, String, f64, f64, f32, String, i32, f32)>::from_arma(data)?;
        Ok(Self {
            uuid,
            r#type,
            point_lat,
            point_lon,
            point_hae,
            contact_callsign,
            track_course,
            track_speed,
            video_url: None,
        })
    }
}

impl MarkerCoTPayload {
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
            track_course: Some(self.track_course),
            track_speed: Some(self.track_speed),
            link_uid: None,
            remarker: None,
            video_url: self.video_url.clone(),
            stale_seconds: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarkerCoTPayload;
    use arma_rs::FromArma;

    #[test]
    fn parses_marker_with_video_and_maps_it_to_cot() {
        let payload = MarkerCoTPayload::from_arma(
            r#"["marker-1","a-f-G-U-C",-30.1,-51.2,100.0,"Falcon",180,10.5,"rtsp://video.example.test:8554/live"]"#
                .to_string(),
        )
        .expect("marker with video should parse");

        assert_eq!(
            payload.video_url.as_deref(),
            Some("rtsp://video.example.test:8554/live")
        );

        let cot = payload.to_cot();
        assert_eq!(cot.uuid.as_deref(), Some("marker-1"));
        assert_eq!(cot.r#type.as_deref(), Some("a-f-G-U-C"));
        assert_eq!(cot.point_lat, -30.1);
        assert_eq!(cot.point_lon, -51.2);
        assert_eq!(cot.point_hae, 100.0);
        assert_eq!(cot.contact_callsign, "Falcon");
        assert_eq!(cot.track_course, Some(180));
        assert_eq!(cot.track_speed, Some(10.5));
        assert_eq!(
            cot.video_url.as_deref(),
            Some("rtsp://video.example.test:8554/live")
        );
    }

    #[test]
    fn treats_blank_video_url_as_absent() {
        let payload = MarkerCoTPayload::from_arma(
            r#"["marker-2","a-f-G-U-C",1,2,3,"Raven",90,5,"   "]"#.to_string(),
        )
        .expect("marker with blank video should parse");

        assert!(payload.video_url.is_none());
    }

    #[test]
    fn accepts_legacy_marker_payload_without_video_field() {
        let payload = MarkerCoTPayload::from_arma(
            r#"["marker-3","a-f-G-U-C",1,2,3,"Viper",45,2.5]"#.to_string(),
        )
        .expect("legacy marker should parse");

        assert_eq!(payload.uuid, "marker-3");
        assert_eq!(payload.r#type, "a-f-G-U-C");
        assert_eq!(payload.contact_callsign, "Viper");
        assert!(payload.video_url.is_none());
    }
}
