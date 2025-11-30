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
