use super::video::video_detail_xml;
use arma_rs::{FromArma, FromArmaError};
use chrono::{Duration, SecondsFormat, Utc};

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
}

fn parse_rtsp_url(url: &str) -> Option<(String, String, String)> {
    let without_proto = url.strip_prefix("rtsp://")?;
    let slash_pos = without_proto.find('/')?;
    let host_port = &without_proto[..slash_pos];
    let path = &without_proto[slash_pos..];
    let colon_pos = host_port.rfind(':')?;
    let address = host_port[..colon_pos].to_string();
    let port = host_port[colon_pos + 1..].to_string();
    Some((address, port, path.to_string()))
}

pub struct UasPlatformCoTPayload {
    pub uid: String,
    pub cot_type: String,
    pub callsign: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub track_course: i32,
    pub track_speed: f32,
    pub sensor_azimuth: i32,
    pub sensor_elevation: i32,
    pub sensor_fov: i32,
    pub sensor_vfov: i32,
    pub sensor_range: i32,
    pub attitude_yaw: i32,
    pub attitude_pitch: f32,
    pub attitude_roll: f32,
    pub hal: f32,
    pub vehicle_type_tag: String,
    pub is_flying: i32,
    pub link_uid: String,
}

impl FromArma for UasPlatformCoTPayload {
    fn from_arma(data: String) -> Result<UasPlatformCoTPayload, FromArmaError> {
        let (
            uid,
            cot_type,
            callsign,
            point_lat,
            point_lon,
            point_hae,
            track_course,
            track_speed,
            sensor_azimuth,
            sensor_elevation,
            sensor_fov,
            sensor_vfov,
            sensor_range,
            attitude_yaw,
            attitude_pitch,
            attitude_roll,
            hal,
            vehicle_type_tag,
            is_flying,
            link_uid,
        ) = <(
            String,
            String,
            String,
            f64,
            f64,
            f32,
            i32,
            f32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            f32,
            f32,
            f32,
            String,
            i32,
            String,
        )>::from_arma(data)?;

        Ok(Self {
            uid,
            cot_type,
            callsign,
            point_lat,
            point_lon,
            point_hae,
            track_course,
            track_speed,
            sensor_azimuth,
            sensor_elevation,
            sensor_fov,
            sensor_vfov,
            sensor_range,
            attitude_yaw,
            attitude_pitch,
            attitude_roll,
            hal,
            vehicle_type_tag,
            is_flying,
            link_uid,
        })
    }
}

impl UasPlatformCoTPayload {
    pub fn to_xml(&self) -> String {
        let uid = escape_xml(&self.uid);
        let cot_type = escape_xml(&self.cot_type);
        let callsign = escape_xml(&self.callsign);
        let link_uid = escape_xml(&self.link_uid);
        let (vehicle_type_tag, video_url) =
            match self.vehicle_type_tag.split_once("|armatak_video_url=") {
                Some((vehicle_type_tag, video_url)) => (
                    escape_xml(vehicle_type_tag),
                    Some(escape_xml(video_url.trim())).filter(|value| !value.is_empty()),
                ),
                None => (escape_xml(&self.vehicle_type_tag), None),
            };
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let stale = (Utc::now() + Duration::milliseconds(3500))
            .to_rfc3339_opts(SecondsFormat::Millis, true);

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>");
        xml.push_str(&format!(
            "<event version=\"2.0\" uid=\"{uid}\" type=\"{cot_type}\" time=\"{now}\" start=\"{now}\" stale=\"{stale}\" how=\"m-g\" access=\"Undefined\">",
            cot_type = cot_type,
            uid = uid,
            now = now,
            stale = stale,
        ));
        xml.push_str(&format!(
            "<point lat=\"{lat}\" lon=\"{lon}\" hae=\"{hae}\" ce=\"9999999.0\" le=\"9999999.0\"/>",
            lat = self.point_lat,
            lon = self.point_lon,
            hae = self.point_hae,
        ));
        xml.push_str("<detail>");
        xml.push_str("<_uastool extendedCot=\"true\" activeRoute=\"false\"/>");
        xml.push_str(&format!(
            "<track course=\"{}\" slope=\"0.0\" speed=\"{}\"/>",
            self.track_course, self.track_speed,
        ));
        xml.push_str(&format!(
            "<sensor elevation=\"{}\" vfov=\"{}\" north=\"0.0\" roll=\"0.0\" range=\"{}\" azimuth=\"{}\" fov=\"{}\" type=\"r-e\" version=\"0.6\"/>",
            self.sensor_elevation,
            self.sensor_vfov,
            self.sensor_range,
            self.sensor_azimuth,
            self.sensor_fov,
        ));
        xml.push_str(&format!(
            "<spatial><attitude roll=\"{}\" pitch=\"{}\" yaw=\"{}\"/><spin roll=\"0.0\" pitch=\"0.0\" yaw=\"0.0\"/></spatial>",
            self.attitude_roll,
            self.attitude_pitch,
            self.attitude_yaw,
        ));
        xml.push_str(&format!(
            "<vehicle goHomeBatteryPercent=\"-2147483648\" hal=\"{}\" flightTimeRemaining=\"-2147483648\" typeTag=\"{}\" batteryRemainingCapacity=\"-2147483648\" isFlying=\"{}\" flightTime=\"-2147483648\" type=\"Generic\" batteryMaxCapacity=\"-2147483648\"/>",
            self.hal,
            vehicle_type_tag,
            if self.is_flying != 0 { "true" } else { "false" },
        ));
        xml.push_str("<_radio rssi=\"-2147483648\" gps=\"false\"/>");
        xml.push_str(&format!("<contact callsign=\"{}\"/>", callsign));
        xml.push_str("<waypointCollection></waypointCollection>");
        xml.push_str(&format!("<_route sender=\"{}\"/>", link_uid));
        xml.push_str("<commandedData climbRate=\"0.0\"/>");
        if let Some(video_url) = video_url {
            xml.push_str(&video_detail_xml(&video_url, &self.uid, &self.callsign));
        } else {
            xml.push_str("<__video></__video>");
        }
        xml.push_str(&format!(
            "<link uid=\"{}\" type=\"a-f-G-U-C\" relation=\"p-p\" />",
            link_uid
        ));
        xml.push_str("</detail></event>");
        xml
    }
}

pub struct UasVideoCoTPayload {
    pub uid: String,
    pub callsign: String,
    pub video_url: String,
}

impl FromArma for UasVideoCoTPayload {
    fn from_arma(data: String) -> Result<UasVideoCoTPayload, FromArmaError> {
        let (uid, callsign, video_url) = <(String, String, String)>::from_arma(data)?;
        Ok(Self {
            uid,
            callsign,
            video_url,
        })
    }
}

impl UasVideoCoTPayload {
    pub fn to_xml(&self) -> String {
        let (address, port, path) = match parse_rtsp_url(&self.video_url) {
            Some(parts) => parts,
            None => {
                log::warn!(
                    "UasVideoCoTPayload: could not parse RTSP URL: {}",
                    self.video_url
                );
                return String::new();
            }
        };
        let callsign = escape_xml(&self.callsign);
        let uid = escape_xml(&self.uid);
        let address = escape_xml(&address);
        let path = escape_xml(&path);

        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let stale =
            (Utc::now() + Duration::seconds(3600)).to_rfc3339_opts(SecondsFormat::Millis, true);

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
        xml.push_str(&format!(
            "<event type=\"b-i-v\" version=\"2.0\" how=\"m-g\" uid=\"{uid}\" time=\"{now}\" start=\"{now}\" stale=\"{stale}\">",
            uid = uid,
            now = now,
            stale = stale
        ));
        xml.push_str(
            "<point lat=\"0\" lon=\"0\" hae=\"9999999.0\" ce=\"9999999.0\" le=\"9999999.0\"/>",
        );
        xml.push_str("<detail>");
        xml.push_str("<__video>");
        xml.push_str(&format!(
            "<ConnectionEntry protocol=\"rtsp\" path=\"{path}\" address=\"{address}\" port=\"{port}\" uid=\"{uid}\" alias=\"{callsign}\" roverPort=\"-1\" rtspReliable=\"0\" ignoreEmbeddedKLV=\"False\" networkTimeout=\"0\" bufferTime=\"-1\"/>",
            path = path,
            address = address,
            port = port,
            uid = uid,
            callsign = callsign,
        ));
        xml.push_str("</__video>");
        xml.push_str(&format!("<contact callsign=\"{}\"/>", callsign));
        xml.push_str("</detail>");
        xml.push_str("</event>");
        xml
    }
}

pub struct UasSensorCoTPayload {
    pub uid: String,
    pub video_uid: String,
    pub callsign: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub azimuth: i32,
    pub fov: i32,
    pub range: i32,
}

impl FromArma for UasSensorCoTPayload {
    fn from_arma(data: String) -> Result<UasSensorCoTPayload, FromArmaError> {
        let (uid, video_uid, callsign, point_lat, point_lon, point_hae, azimuth, fov, range) =
            <(String, String, String, f64, f64, f32, i32, i32, i32)>::from_arma(data)?;
        Ok(Self {
            uid,
            video_uid,
            callsign,
            point_lat,
            point_lon,
            point_hae,
            azimuth,
            fov,
            range,
        })
    }
}

impl UasSensorCoTPayload {
    pub fn to_xml(&self) -> String {
        let uid = escape_xml(&self.uid);
        let video_uid = escape_xml(&self.video_uid);
        let callsign = escape_xml(&self.callsign);
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let stale =
            (Utc::now() + Duration::seconds(60)).to_rfc3339_opts(SecondsFormat::Millis, true);

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
        xml.push_str(&format!(
            "<event type=\"b-m-p-s-p-loc\" version=\"2.0\" how=\"h-g-i-g-o\" uid=\"{uid}\" time=\"{now}\" start=\"{now}\" stale=\"{stale}\">",
            uid = uid,
            now = now,
            stale = stale,
        ));
        xml.push_str(&format!(
            "<point lat=\"{lat}\" lon=\"{lon}\" hae=\"{hae}\" ce=\"9999999.0\" le=\"9999999.0\"/>",
            lat = self.point_lat,
            lon = self.point_lon,
            hae = self.point_hae,
        ));
        xml.push_str("<detail>");
        xml.push_str(&format!(
            "<sensor fov=\"{fov}\" fovRed=\"1\" fovGreen=\"1\" fovBlue=\"1\" fovAlpha=\"0.5372549\" displayMagneticReference=\"0\" range=\"{range}\" azimuth=\"{az}\"/>",
            fov = self.fov,
            range = self.range,
            az = self.azimuth,
        ));
        xml.push_str(&format!("<__video uid=\"{}\"/>", video_uid));
        xml.push_str(&format!("<contact callsign=\"{}\"/>", callsign));
        xml.push_str("</detail>");
        xml.push_str("</event>");
        xml
    }
}
