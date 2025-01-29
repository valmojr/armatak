use arma_rs::{FromArma, FromArmaError};
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

pub struct HumanCoTPayload {
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

impl FromArma for HumanCoTPayload {
    fn from_arma(data: String) -> Result<HumanCoTPayload, FromArmaError> {
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

impl HumanCoTPayload {
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
        }
    }
}

pub struct MarkerCoTPayload {
    pub uuid: String,
    pub r#type: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub contact_callsign: String,
    pub track_course: i32,
    pub track_speed: f32,
}

impl FromArma for MarkerCoTPayload {
    fn from_arma(data: String) -> Result<MarkerCoTPayload, FromArmaError> {
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
        }
    }
}
