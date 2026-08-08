use super::video::video_detail_xml;
use chrono::{Duration, SecondsFormat, Utc};
use uuid::Uuid;

pub struct CursorOverTime {
    pub uuid: Option<String>,
    pub r#type: Option<String>,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub point_ce: Option<f32>,
    pub point_le: Option<f32>,
    pub contact_callsign: String,
    pub group_name: Option<String>,
    pub group_role: Option<String>,
    pub track_course: Option<i32>,
    pub track_speed: Option<f32>,
    pub link_uid: Option<String>,
    pub remarker: Option<String>,
    pub video_url: Option<String>,
    pub stale_seconds: Option<i64>,
}

impl CursorOverTime {
    pub fn convert_to_xml(&self) -> String {
        let uuid = match &self.uuid {
            Some(uuid) => uuid,
            None => &Uuid::new_v4().to_string(),
        };

        let marker_type = match &self.r#type {
            Some(marker_type) => marker_type,
            None => &"a-f-G-U-C-I".to_string(),
        };

        let created_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let stale_seconds = self.stale_seconds.unwrap_or(360).max(1);
        let stale_time = (Utc::now() + Duration::seconds(stale_seconds))
            .to_rfc3339_opts(SecondsFormat::Millis, true);

        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");

        xml.push_str(
        format!("<event type=\"{}\" version=\"2.0\" how=\"m-g\" uid=\"{}\" time=\"{}\" start=\"{}\" stale=\"{}\">",
        marker_type, uuid, created_time, created_time, stale_time).as_str());

        let point_ce = match &self.point_ce {
            Some(point_ce) => point_ce,
            None => &9999999.0,
        };

        let point_le = match &self.point_le {
            Some(point_le) => point_le,
            None => &9999999.0,
        };

        xml.push_str(
            format!(
                "<point ce=\"{}\" le=\"{}\" hae=\"{}\" lat=\"{}\" lon=\"{}\" />",
                point_ce, point_le, self.point_hae, self.point_lat, self.point_lon
            )
            .as_str(),
        );

        xml.push_str("<detail>");

        xml.push_str("<takv device=\"Samsung S24\" os=\"Arma 3\" platform=\"ARMATAK\" version=\"0.9.0.0\" />");

        if let Some(linked_uid) = &self.link_uid {
            xml.push_str("<precisionlocation altsrc=\"DTED0\" />");
            xml.push_str(
                format!(
                    "<link uid=\"{}\" type=\"a-f-G-U-C\" relation=\"p-p\" />",
                    linked_uid,
                )
                .as_str(),
            );
            xml.push_str("<hideLabel />");
        }

        xml.push_str(format!("<contact callsign=\"{}\" />", self.contact_callsign).as_str());

        xml.push_str(format!("<uid Droid=\"{}\"/>", self.contact_callsign).as_str());

        if let (Some(track_course), Some(track_speed)) = (&self.track_course, &self.track_speed) {
            xml.push_str(
                format!(
                    "<track course=\"{}\" speed=\"{}\" />",
                    track_course, track_speed
                )
                .as_str(),
            );

            xml.push_str("<status battery=\"89\" />");
        }

        if let (Some(group_name), Some(group_role)) = (&self.group_name, &self.group_role) {
            xml.push_str(
                format!(
                    "<__group name=\"{}\" role=\"{}\" />",
                    group_name, group_role
                )
                .as_str(),
            );
        }

        if let Some(remark) = &self.remarker {
            xml.push_str(format!("<remarks>ARMATAK | {}</remarks>", remark).as_str());
        }

        if let Some(video_url) = &self.video_url {
            if !video_url.trim().is_empty() {
                xml.push_str(&video_detail_xml(video_url, uuid, &self.contact_callsign));
            }
        }

        xml.push_str("</detail></event>");

        return xml;
    }
}

#[cfg(test)]
mod tests {
    use super::CursorOverTime;

    fn minimal_marker() -> CursorOverTime {
        CursorOverTime {
            uuid: None,
            r#type: None,
            point_lat: -30.0,
            point_lon: -51.0,
            point_hae: 10.0,
            point_ce: None,
            point_le: None,
            contact_callsign: "Falcon".to_string(),
            group_name: None,
            group_role: None,
            track_course: None,
            track_speed: None,
            link_uid: None,
            remarker: None,
            video_url: None,
            stale_seconds: None,
        }
    }

    #[test]
    fn serializes_minimal_marker_with_tak_defaults() {
        let xml = minimal_marker().convert_to_xml();

        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>"));
        assert!(xml.contains("<event type=\"a-f-G-U-C-I\" version=\"2.0\""));
        assert!(xml.contains("<point ce=\"9999999\" le=\"9999999\" hae=\"10\" lat=\"-30\" lon=\"-51\" />"));
        assert!(xml.contains("<contact callsign=\"Falcon\" />"));
        assert!(xml.contains("<uid Droid=\"Falcon\"/>"));
        assert!(!xml.contains("<link "));
        assert!(!xml.contains("<track "));
        assert!(!xml.contains("<__group "));
        assert!(!xml.contains("<remarks>"));
        assert!(!xml.contains("<__video>"));
        assert!(xml.ends_with("</detail></event>"));
    }

    #[test]
    fn serializes_all_optional_marker_details() {
        let marker = CursorOverTime {
            uuid: Some("marker-1".to_string()),
            r#type: Some("a-f-G-U-C".to_string()),
            point_lat: 1.25,
            point_lon: 2.5,
            point_hae: 100.0,
            point_ce: Some(3.0),
            point_le: Some(4.0),
            contact_callsign: "Raven".to_string(),
            group_name: Some("Cyan".to_string()),
            group_role: Some("Team Member".to_string()),
            track_course: Some(180),
            track_speed: Some(12.5),
            link_uid: Some("parent-1".to_string()),
            remarker: Some("test remark".to_string()),
            video_url: Some("rtsp://video.example.test:8554/live".to_string()),
            stale_seconds: Some(0),
        };

        let xml = marker.convert_to_xml();

        assert!(xml.contains("type=\"a-f-G-U-C\""));
        assert!(xml.contains("uid=\"marker-1\""));
        assert!(xml.contains("<point ce=\"3\" le=\"4\" hae=\"100\" lat=\"1.25\" lon=\"2.5\" />"));
        assert!(xml.contains("<precisionlocation altsrc=\"DTED0\" />"));
        assert!(xml.contains("<link uid=\"parent-1\" type=\"a-f-G-U-C\" relation=\"p-p\" />"));
        assert!(xml.contains("<hideLabel />"));
        assert!(xml.contains("<track course=\"180\" speed=\"12.5\" />"));
        assert!(xml.contains("<status battery=\"89\" />"));
        assert!(xml.contains("<__group name=\"Cyan\" role=\"Team Member\" />"));
        assert!(xml.contains("<remarks>ARMATAK | test remark</remarks>"));
        assert!(xml.contains("<ConnectionEntry protocol=\"rtsp\""));
    }
}
