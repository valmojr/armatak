use arma_rs::{FromArma, FromArmaError};

#[derive(Clone)]
struct DrawPoint {
    lat: f64,
    lon: f64,
    hae: f32,
}

pub struct DrawEllipsePayload {
    pub uuid: String,
    pub cot_type: String,
    pub center_lat: f64,
    pub center_lon: f64,
    pub center_hae: f32,
    pub major: f64,
    pub minor: f64,
    pub angle: f32,
    pub callsign: String,
    pub stale_seconds: i64,
    pub stroke_color: i32,
    pub fill_color: i32,
    pub stroke_weight: f64,
    pub milsym: String,
}

pub struct DrawLinksPayload {
    pub uuid: String,
    pub cot_type: String,
    pub center_lat: f64,
    pub center_lon: f64,
    pub center_hae: f32,
    pub points: String,
    pub callsign: String,
    pub stale_seconds: i64,
    pub stroke_color: i32,
    pub fill_color: i32,
    pub stroke_weight: f64,
    pub stroke_style: String,
    pub closed: bool,
    pub milsym: String,
}

impl FromArma for DrawEllipsePayload {
    fn from_arma(data: String) -> Result<Self, FromArmaError> {
        let (
            uuid,
            cot_type,
            center_lat,
            center_lon,
            center_hae,
            major,
            minor,
            angle,
            callsign,
            stale_seconds,
            stroke_color,
            fill_color,
            stroke_weight,
            milsym,
        ) = <(
            String,
            String,
            f64,
            f64,
            f32,
            f64,
            f64,
            f32,
            String,
            i64,
            i32,
            i32,
            f64,
            String,
        )>::from_arma(data)?;

        Ok(Self {
            uuid,
            cot_type,
            center_lat,
            center_lon,
            center_hae,
            major,
            minor,
            angle,
            callsign,
            stale_seconds,
            stroke_color,
            fill_color,
            stroke_weight,
            milsym,
        })
    }
}

impl FromArma for DrawLinksPayload {
    fn from_arma(data: String) -> Result<Self, FromArmaError> {
        let (
            uuid,
            cot_type,
            center_lat,
            center_lon,
            center_hae,
            points,
            callsign,
            stale_seconds,
            stroke_color,
            fill_color,
            stroke_weight,
            stroke_style,
            closed,
            milsym,
        ) = <(
            String,
            String,
            f64,
            f64,
            f32,
            String,
            String,
            i64,
            i32,
            i32,
            f64,
            String,
            bool,
            String,
        )>::from_arma(data)?;

        Ok(Self {
            uuid,
            cot_type,
            center_lat,
            center_lon,
            center_hae,
            points,
            callsign,
            stale_seconds,
            stroke_color,
            fill_color,
            stroke_weight,
            stroke_style,
            closed,
            milsym,
        })
    }
}

impl DrawEllipsePayload {
    pub fn to_xml(&self, now: &str, stale: &str) -> String {
        shape_event(
            &self.uuid,
            &self.cot_type,
            self.center_lat,
            self.center_lon,
            self.center_hae,
            now,
            stale,
            &self.callsign,
            &ellipse_shape_detail(
                &self.uuid,
                self.major,
                self.minor,
                self.angle,
                self.stroke_color,
                self.fill_color,
                self.stroke_weight,
            ),
            self.stroke_color,
            self.fill_color,
            self.stroke_weight,
            "solid",
            &self.milsym,
        )
    }
}

impl DrawLinksPayload {
    pub fn to_xml(&self, now: &str, stale: &str) -> String {
        let points = parse_points(&self.points);
        let shape_detail = links_detail(&points, self.closed);

        shape_event(
            &self.uuid,
            &self.cot_type,
            self.center_lat,
            self.center_lon,
            self.center_hae,
            now,
            stale,
            &self.callsign,
            &shape_detail,
            self.stroke_color,
            self.fill_color,
            self.stroke_weight,
            &self.stroke_style,
            &self.milsym,
        )
    }
}

fn shape_event(
    uid: &str,
    cot_type: &str,
    lat: f64,
    lon: f64,
    hae: f32,
    now: &str,
    stale: &str,
    callsign: &str,
    shape_detail: &str,
    stroke_color: i32,
    fill_color: i32,
    stroke_weight: f64,
    stroke_style: &str,
    milsym: &str,
) -> String {
    let milsym_detail = if milsym.trim().is_empty() {
        String::new()
    } else {
        format!(r#"<__milsym id="{}" />"#, escape_attr(milsym))
    };

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" ?><event version="2.0" uid="{uid}" type="{cot_type}" time="{now}" start="{now}" stale="{stale}" how="h-e" access="Undefined"><point lat="{lat}" lon="{lon}" hae="{hae}" ce="10.9" le="9999999.0" /><detail>{shape_detail}<__shapeExtras cpvis="true" editable="true" /><contact callsign="{callsign}" /><remarks /><archive /><labels_on value="true" /><strokeColor value="{stroke_color}" /><strokeWeight value="{stroke_weight}" /><strokeStyle value="{stroke_style}" /><fillColor value="{fill_color}" /><precisionlocation altsrc="GPS" geopointsrc="GPS" />{milsym_detail}</detail></event>"#,
        uid = escape_attr(uid),
        cot_type = escape_attr(cot_type),
        now = now,
        stale = stale,
        lat = lat,
        lon = lon,
        hae = hae,
        shape_detail = shape_detail,
        callsign = escape_attr(callsign),
        stroke_color = stroke_color,
        stroke_weight = stroke_weight,
        stroke_style = escape_attr(stroke_style),
        fill_color = fill_color,
        milsym_detail = milsym_detail
    )
}

fn ellipse_shape_detail(
    uid: &str,
    major: f64,
    minor: f64,
    angle: f32,
    stroke_color: i32,
    fill_color: i32,
    stroke_weight: f64,
) -> String {
    format!(
        r#"<shape><ellipse major="{major}" minor="{minor}" angle="{angle}" /><link uid="{style_uid}" type="b-x-KmlStyle" relation="p-c"><Style><LineStyle><color>{stroke_hex}</color><width>{stroke_weight}</width></LineStyle><PolyStyle><color>{fill_hex}</color></PolyStyle></Style></link></shape>"#,
        major = major,
        minor = minor,
        angle = angle,
        style_uid = escape_attr(&format!("{}.Style", uid)),
        stroke_hex = argb_hex(stroke_color),
        stroke_weight = stroke_weight,
        fill_hex = argb_hex(fill_color)
    )
}

fn links_detail(points: &[DrawPoint], closed: bool) -> String {
    let mut detail = String::new();

    for point in points {
        detail.push_str(&link_detail(point));
    }

    if closed {
        if let Some(first) = points.first() {
            detail.push_str(&link_detail(first));
        }
    }

    detail
}

fn link_detail(point: &DrawPoint) -> String {
    format!(
        r#"<link point="{lat},{lon},{hae}" />"#,
        lat = point.lat,
        lon = point.lon,
        hae = point.hae
    )
}

fn parse_points(raw: &str) -> Vec<DrawPoint> {
    raw.split(';')
        .filter_map(|entry| {
            let parts: Vec<_> = entry.split(',').collect();
            if parts.len() != 3 {
                return None;
            }

            Some(DrawPoint {
                lat: parts[0].parse().ok()?,
                lon: parts[1].parse().ok()?,
                hae: parts[2].parse().ok()?,
            })
        })
        .collect()
}

fn argb_hex(color: i32) -> String {
    format!("{:08x}", color as u32)
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::{parse_points, DrawEllipsePayload, DrawLinksPayload};
    use arma_rs::FromArma;

    const NOW: &str = "2026-08-07T17:00:00.000Z";
    const STALE: &str = "2026-08-07T18:00:00.000Z";

    #[test]
    fn parses_and_serializes_ellipse_shape_with_milsym() {
        let payload = DrawEllipsePayload::from_arma(
            r#"["ellipse<&","u-d-c-e",-30.1,-51.2,10,100,50,25,"Ellipse<&",600,-1,-1761607681,3,"SFGPUCI----K<&"]"#
                .to_string(),
        )
        .expect("ellipse payload should parse");

        assert_eq!(payload.uuid, "ellipse<&");
        assert_eq!(payload.stale_seconds, 600);
        assert_eq!(payload.major, 100.0);
        assert_eq!(payload.minor, 50.0);

        let xml = payload.to_xml(NOW, STALE);
        assert!(xml.contains("uid=\"ellipse&lt;&amp;\" type=\"u-d-c-e\""));
        assert!(xml.contains("<ellipse major=\"100\" minor=\"50\" angle=\"25\" />"));
        assert!(xml.contains("<color>ffffffff</color>"));
        assert!(xml.contains("<color>96ffffff</color>"));
        assert!(xml.contains("<contact callsign=\"Ellipse&lt;&amp;\" />"));
        assert!(xml.contains("<__milsym id=\"SFGPUCI----K&lt;&amp;\" />"));
    }

    #[test]
    fn parses_and_serializes_closed_link_shape_without_milsym() {
        let payload = DrawLinksPayload::from_arma(
            r#"["shape<&","u-d-f",1,2,3,"1,2,3;4,5,6","Polygon<&",300,-16777216,16777215,2,"dashed<&",true,""]"#
                .to_string(),
        )
        .expect("link shape payload should parse");

        assert!(payload.closed);
        assert_eq!(payload.stroke_style, "dashed<&");

        let xml = payload.to_xml(NOW, STALE);
        assert!(xml.contains("uid=\"shape&lt;&amp;\" type=\"u-d-f\""));
        assert_eq!(xml.matches("<link point=\"1,2,3\" />").count(), 2);
        assert_eq!(xml.matches("<link point=\"4,5,6\" />").count(), 1);
        assert!(xml.contains("<strokeStyle value=\"dashed&lt;&amp;\" />"));
        assert!(!xml.contains("<__milsym"));
    }

    #[test]
    fn point_parser_drops_invalid_shape_points() {
        let points = parse_points("1,2,3;missing;bad,2,3;4,bad,6;7,8,bad;9,10,11");
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].lat, 1.0);
        assert_eq!(points[1].lon, 10.0);
    }
}
