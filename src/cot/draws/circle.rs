use arma_rs::{FromArma, FromArmaError};

pub struct CircleCoTPayload {
    pub uuid: String,
    pub center_lat: f64,
    pub center_lon: f64,
    pub center_hae: f32,
    pub major: f64,
    pub minor: f64,
    pub angle: f32,
    pub callsign: String,
    pub creator_uid: String,
    pub creator_callsign: String,
}

impl FromArma for CircleCoTPayload {
    fn from_arma(data: String) -> Result<Self, FromArmaError> {
        let (
            uuid,
            center_lat,
            center_lon,
            center_hae,
            major,
            minor,
            angle,
            callsign,
            creator_uid,
            creator_callsign,
        ) = <(String, f64, f64, f32, f64, f64, f32, String, String, String)>::from_arma(data)?;

        Ok(Self {
            uuid,
            center_lat,
            center_lon,
            center_hae,
            major,
            minor,
            angle,
            callsign,
            creator_uid,
            creator_callsign,
        })
    }
}

pub struct ShapeCircleCoT {
    pub uid: String,
    pub lat: f64,
    pub lon: f64,
    pub hae: f32,
    pub major: f64,
    pub minor: f64,
    pub angle: f32,
    pub callsign: String,
    pub creator_uid: String,
    pub creator_callsign: String,
}

impl CircleCoTPayload {
    pub fn to_cot(&self) -> ShapeCircleCoT {
        ShapeCircleCoT {
            uid: self.uuid.clone(),
            lat: self.center_lat,
            lon: self.center_lon,
            hae: self.center_hae,
            major: self.major,
            minor: self.minor,
            angle: self.angle,
            callsign: self.callsign.clone(),
            creator_uid: self.creator_uid.clone(),
            creator_callsign: self.creator_callsign.clone(),
        }
    }
}

impl ShapeCircleCoT {
    pub fn to_xml(&self, now: &str, stale: &str) -> String {
        format!(
            r#"<event version="2.0" uid="{uid}" type="u-d-c-c"
  time="{t}" start="{t}" stale="{stale}"
  how="h-e" access="Undefined">
  <point lat="{lat}" lon="{lon}" hae="{hae}" ce="10.9" le="9999999.0" />
  <detail>
    <shape>
      <ellipse major="{major}" minor="{minor}" angle="{angle}" />
      <link uid="{uid}.Style" type="b-x-KmlStyle" relation="p-c">
        <Style>
          <LineStyle>
            <color>ffffffff</color>
            <width>3.0</width>
          </LineStyle>
          <PolyStyle>
            <color>96ffffff</color>
          </PolyStyle>
        </Style>
      </link>
      <link uid="{creator_uid}" type="self" relation="p-p-CenterAnchor" />
    </shape>
    <__shapeExtras cpvis="true" editable="true" />
    <remarks />
    <contact callsign="{callsign}" />
    <creator uid="{creator_uid}" callsign="{creator_callsign}" time="{t}" type="a-f-G-U-C" />
    <archive />
    <labels_on value="true" />
    <strokeColor value="-1" />
    <strokeWeight value="3.0" />
    <strokeStyle value="solid" />
    <fillColor value="-1761607681" />
    <precisionlocation altsrc="GPS" geopointsrc="GPS" />
  </detail>
</event>"#,
            uid = self.uid,
            t = now,
            stale = stale,
            lat = self.lat,
            lon = self.lon,
            hae = self.hae,
            major = self.major,
            minor = self.minor,
            angle = self.angle,
            callsign = self.callsign,
            creator_uid = self.creator_uid,
            creator_callsign = self.creator_callsign
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CircleCoTPayload;
    use arma_rs::FromArma;

    #[test]
    fn parses_circle_payload_maps_shape_and_serializes_cot() {
        let payload = CircleCoTPayload::from_arma(
            r#"["circle-1",-30.1,-51.2,20,100,50,30,"Area Alpha","creator-1","Falcon"]"#
                .to_string(),
        )
        .expect("circle payload should parse");

        assert_eq!(payload.uuid, "circle-1");
        assert_eq!(payload.center_lat, -30.1);
        assert_eq!(payload.center_lon, -51.2);
        assert_eq!(payload.center_hae, 20.0);
        assert_eq!(payload.major, 100.0);
        assert_eq!(payload.minor, 50.0);
        assert_eq!(payload.angle, 30.0);

        let shape = payload.to_cot();
        assert_eq!(shape.uid, "circle-1");
        assert_eq!(shape.callsign, "Area Alpha");
        assert_eq!(shape.creator_uid, "creator-1");
        assert_eq!(shape.creator_callsign, "Falcon");

        let xml = shape.to_xml("2026-08-07T17:00:00.000Z", "2026-08-07T18:00:00.000Z");
        assert!(xml.contains("uid=\"circle-1\" type=\"u-d-c-c\""));
        assert!(xml.contains("<point lat=\"-30.1\" lon=\"-51.2\" hae=\"20\""));
        assert!(xml.contains("<ellipse major=\"100\" minor=\"50\" angle=\"30\" />"));
        assert!(xml.contains("<link uid=\"creator-1\" type=\"self\" relation=\"p-p-CenterAnchor\" />"));
        assert!(xml.contains("<contact callsign=\"Area Alpha\" />"));
        assert!(xml.contains("<creator uid=\"creator-1\" callsign=\"Falcon\""));
    }
}
