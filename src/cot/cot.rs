use uuid::Uuid;
use chrono::{Duration, SecondsFormat, Utc};

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

        let stale_time =
            (Utc::now() + Duration::seconds(360)).to_rfc3339_opts(SecondsFormat::Millis, true);

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

        xml.push_str("</detail></event>");

        return xml;
    }
}
