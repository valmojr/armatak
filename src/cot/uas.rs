// src/cot/uas.rs
//
// CoT types required for ATAK UAS Tool integration.
//
// Two event types are needed so that the UAS Tool plugin recognises a drone:
//
//   b-i-v              — Video endpoint declaration. Tells the UAS Tool where
//                        to pull the RTSP stream for this drone.
//
//   b-m-p-s-p-loc      — Sensor position event. Carries the camera azimuth,
//                        field-of-view, and slant-range that the UAS Tool uses
//                        to draw the FOV cone on the map and to project AR
//                        markers onto the video feed.
//
// The two events are linked: the b-m-p-s-p-loc detail contains
//   <__video uid="<drone-uuid>"/>
// which references the uid of the b-i-v event, so the UAS Tool knows which
// video stream belongs to this sensor.

use arma_rs::{FromArma, FromArmaError};
use chrono::{Duration, SecondsFormat, Utc};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse an RTSP URL of the form  rtsp://address:port/path
/// into its three components.
fn parse_rtsp_url(url: &str) -> Option<(String, String, String)> {
    let without_proto = url.strip_prefix("rtsp://")?;
    let slash_pos = without_proto.find('/')?;
    let host_port = &without_proto[..slash_pos];
    let path = &without_proto[slash_pos..]; // includes the leading '/'
    let colon_pos = host_port.rfind(':')?;
    let address = host_port[..colon_pos].to_string();
    let port = host_port[colon_pos + 1..].to_string();
    Some((address, port, path.to_string()))
}

// ---------------------------------------------------------------------------
// b-i-v  –  Video endpoint declaration
// ---------------------------------------------------------------------------

pub struct UasVideoCoTPayload {
    /// The drone's persistent ATAK UUID (same uid used for PPLI / marker CoT).
    pub uid: String,
    /// Human-readable label shown in the UAS Tool video list.
    pub callsign: String,
    /// Full RTSP URL, e.g. "rtsp://192.168.1.10:8554/live/drone1".
    pub video_url: String,
}

impl FromArma for UasVideoCoTPayload {
    fn from_arma(data: String) -> Result<UasVideoCoTPayload, FromArmaError> {
        let (uid, callsign, video_url) =
            <(String, String, String)>::from_arma(data)?;
        Ok(Self {
            uid,
            callsign,
            video_url,
        })
    }
}

impl UasVideoCoTPayload {
    /// Build the complete XML string for the b-i-v CoT event.
    /// Returns an empty string if the RTSP URL cannot be parsed.
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

        let now =
            Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        // Long stale time: the video endpoint is considered valid for 1 hour.
        // The CoT is re-sent every router tick so it stays fresh even if the
        // TAK server restarts.
        let stale = (Utc::now() + Duration::seconds(3600))
            .to_rfc3339_opts(SecondsFormat::Millis, true);

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
        xml.push_str(&format!(
            "<event type=\"b-i-v\" version=\"2.0\" how=\"m-g\" \
             uid=\"{uid}\" time=\"{now}\" start=\"{now}\" stale=\"{stale}\">",
            uid = self.uid,
            now = now,
            stale = stale
        ));
        // b-i-v events carry no real geographic position.
        xml.push_str(
            "<point lat=\"0\" lon=\"0\" hae=\"9999999.0\" \
             ce=\"9999999.0\" le=\"9999999.0\"/>",
        );
        xml.push_str("<detail>");
        xml.push_str("<__video>");
        xml.push_str(&format!(
            "<ConnectionEntry \
             protocol=\"rtsp\" \
             path=\"{path}\" \
             address=\"{address}\" \
             port=\"{port}\" \
             uid=\"{uid}\" \
             alias=\"{callsign}\" \
             roverPort=\"-1\" \
             rtspReliable=\"0\" \
             ignoreEmbeddedKLV=\"False\" \
             networkTimeout=\"0\" \
             bufferTime=\"-1\"/>",
            path = path,
            address = address,
            port = port,
            uid = self.uid,
            callsign = self.callsign,
        ));
        xml.push_str("</__video>");
        xml.push_str(&format!(
            "<contact callsign=\"{}\"/>",
            self.callsign
        ));
        xml.push_str("</detail>");
        xml.push_str("</event>");
        xml
    }
}

// ---------------------------------------------------------------------------
// b-m-p-s-p-loc  –  Sensor position (FOV cone + video link)
// ---------------------------------------------------------------------------

pub struct UasSensorCoTPayload {
    /// UID for this sensor event — conventionally "<drone-uuid>-sensor".
    pub uid: String,
    /// The drone's ATAK UUID; must match the uid used in the b-i-v event so
    /// the UAS Tool can link sensor data to the correct video stream.
    pub video_uid: String,
    /// Callsign shown in the UAS Tool sensor list.
    pub callsign: String,
    /// Drone latitude in decimal degrees (WGS-84).
    pub point_lat: f64,
    /// Drone longitude in decimal degrees (WGS-84).
    pub point_lon: f64,
    /// Drone height above ellipsoid in metres (WGS-84).
    pub point_hae: f32,
    /// Camera azimuth in degrees, clockwise from true North (0–359).
    pub azimuth: i32,
    /// Camera horizontal field of view in degrees.
    pub fov: i32,
    /// Estimated slant range from drone to ground point in metres.
    /// A good approximation is the drone's AGL altitude.
    pub range: i32,
}

impl FromArma for UasSensorCoTPayload {
    fn from_arma(data: String) -> Result<UasSensorCoTPayload, FromArmaError> {
        let (
            uid,
            video_uid,
            callsign,
            point_lat,
            point_lon,
            point_hae,
            azimuth,
            fov,
            range,
        ) = <(String, String, String, f64, f64, f32, i32, i32, i32)>::from_arma(
            data,
        )?;
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
    /// Build the complete XML string for the b-m-p-s-p-loc CoT event.
    pub fn to_xml(&self) -> String {
        let now =
            Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        // 60-second stale: must be refreshed every router tick (1 s) to keep
        // the FOV cone visible on the map.
        let stale = (Utc::now() + Duration::seconds(60))
            .to_rfc3339_opts(SecondsFormat::Millis, true);

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
        xml.push_str(&format!(
            "<event type=\"b-m-p-s-p-loc\" version=\"2.0\" how=\"h-g-i-g-o\" \
             uid=\"{uid}\" time=\"{now}\" start=\"{now}\" stale=\"{stale}\">",
            uid = self.uid,
            now = now,
            stale = stale,
        ));
        xml.push_str(&format!(
            "<point lat=\"{lat}\" lon=\"{lon}\" hae=\"{hae}\" \
             ce=\"9999999.0\" le=\"9999999.0\"/>",
            lat = self.point_lat,
            lon = self.point_lon,
            hae = self.point_hae,
        ));
        xml.push_str("<detail>");
        // fovAlpha controls the transparency of the FOV cone fill (0–1).
        // 0.537 ≈ 137/255, the value used by the real UAS Tool.
        xml.push_str(&format!(
            "<sensor \
             fov=\"{fov}\" \
             fovRed=\"1\" \
             fovGreen=\"1\" \
             fovBlue=\"1\" \
             fovAlpha=\"0.5372549\" \
             displayMagneticReference=\"0\" \
             range=\"{range}\" \
             azimuth=\"{az}\"/>",
            fov = self.fov,
            range = self.range,
            az = self.azimuth,
        ));
        // Link this sensor event to the b-i-v video endpoint.
        xml.push_str(&format!("<__video uid=\"{}\"/>", self.video_uid));
        xml.push_str(&format!(
            "<contact callsign=\"{}\"/>",
            self.callsign
        ));
        xml.push_str("</detail>");
        xml.push_str("</event>");
        xml
    }
}
