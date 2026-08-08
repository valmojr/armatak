use arma_rs::{FromArma, FromArmaError};
use uuid::Uuid;

#[derive(Clone)]
struct RoutePoint {
    lat: f64,
    lon: f64,
    hae: f32,
}

pub struct RoutePayload {
    pub uuid: String,
    pub points: String,
    pub callsign: String,
    pub stale_seconds: i64,
    pub color: i32,
    pub stroke_weight: f64,
    pub method: String,
    pub route_type: String,
    pub direction: String,
    pub checkpoint_interval: usize,
}

impl FromArma for RoutePayload {
    fn from_arma(data: String) -> Result<Self, FromArmaError> {
        let (
            uuid,
            points,
            callsign,
            stale_seconds,
            color,
            stroke_weight,
            method,
            route_type,
            direction,
            checkpoint_interval,
        ) = <(
            String,
            String,
            String,
            i64,
            i32,
            f64,
            String,
            String,
            String,
            i32,
        )>::from_arma(data)?;

        Ok(Self {
            uuid,
            points,
            callsign,
            stale_seconds,
            color,
            stroke_weight,
            method,
            route_type,
            direction,
            checkpoint_interval: checkpoint_interval.max(1) as usize,
        })
    }
}

impl RoutePayload {
    pub fn to_xml(&self, now: &str, stale: &str) -> String {
        let points = parse_points(&self.points);
        let links = route_links(&self.callsign, &points, self.checkpoint_interval);

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" ?><event version="2.0" uid="{uid}" type="b-m-r" time="{now}" start="{now}" stale="{stale}" how="h-e" access="Undefined"><point lat="0.0" lon="0.0" hae="9999999.0" ce="9999999.0" le="9999999.0"/><detail>{links}<link_attr planningmethod="{direction}" color="{color}" method="{method}" prefix="CP" style="0" type="Vehicle" stroke="{stroke_weight}" direction="{direction}" routetype="{route_type}" order="Ascending Check Points"/><creator uid="ARMATAK" callsign="ArmaTAK" time="{now}" type="a-f-G-U-C"/><strokeColor value="{color}"/><strokeWeight value="{stroke_weight}"/><strokeStyle value="solid"/><labels_on value="false"/><__routeinfo><__navcues/></__routeinfo><color value="{color}"/><remarks/><contact callsign="{callsign}"/><archive/><height_unit>1</height_unit></detail></event>"#,
            uid = escape_attr(&self.uuid),
            now = now,
            stale = stale,
            links = links,
            direction = escape_attr(&self.direction),
            color = self.color,
            method = escape_attr(&self.method),
            stroke_weight = self.stroke_weight.max(1.0),
            route_type = escape_attr(&self.route_type),
            callsign = escape_attr(&self.callsign)
        )
    }
}

fn route_links(callsign: &str, points: &[RoutePoint], checkpoint_interval: usize) -> String {
    let mut xml = String::new();
    let last_index = points.len().saturating_sub(1);
    let mut checkpoint_number = 1;

    for (index, point) in points.iter().enumerate() {
        let is_start = index == 0;
        let is_end = index == last_index;
        let is_checkpoint = !is_start && !is_end && index % checkpoint_interval == 0;
        let point_callsign = if is_start {
            format!("{} SP", callsign)
        } else if is_end {
            "VDO".to_string()
        } else if is_checkpoint {
            let name = format!("CP{}", checkpoint_number);
            checkpoint_number += 1;
            name
        } else {
            String::new()
        };
        let link_type = if point_callsign.is_empty() {
            "b-m-p-c"
        } else {
            "b-m-p-w"
        };

        xml.push_str(&format!(
            r#"<link uid="{link_uid}" callsign="{callsign}" type="{link_type}" point="{lat},{lon},{hae}" remarks="" relation="c"/>"#,
            link_uid = escape_attr(&Uuid::new_v4().to_string()),
            callsign = escape_attr(&point_callsign),
            link_type = link_type,
            lat = point.lat,
            lon = point.lon,
            hae = point.hae
        ));
    }

    xml
}

fn parse_points(raw: &str) -> Vec<RoutePoint> {
    raw.split(';')
        .filter_map(|entry| {
            let parts: Vec<_> = entry.split(',').collect();
            if parts.len() != 3 {
                return None;
            }

            Some(RoutePoint {
                lat: parts[0].parse().ok()?,
                lon: parts[1].parse().ok()?,
                hae: parts[2].parse().ok()?,
            })
        })
        .collect()
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
    use super::{parse_points, route_links, RoutePayload, RoutePoint};
    use arma_rs::FromArma;

    #[test]
    fn parses_route_payload_clamps_interval_and_serializes_attributes() {
        let mut payload = RoutePayload::from_arma(
            r#"["route<&","","Route<&",600,-1,0.5,"Driving<&","Primary<&","Infil<&",0]"#
                .to_string(),
        )
        .expect("route payload should parse");
        payload.points = "-30.1,-51.2,10;-30.2,-51.3,20".to_string();

        assert_eq!(payload.checkpoint_interval, 1);
        assert_eq!(payload.stale_seconds, 600);
        assert_eq!(payload.stroke_weight, 0.5);

        let xml = payload.to_xml("2026-08-07T17:00:00.000Z", "2026-08-07T18:00:00.000Z");
        assert!(xml.contains("uid=\"route&lt;&amp;\" type=\"b-m-r\""));
        assert!(xml.contains("planningmethod=\"Infil&lt;&amp;\""));
        assert!(xml.contains("method=\"Driving&lt;&amp;\""));
        assert!(xml.contains("routetype=\"Primary&lt;&amp;\""));
        assert!(xml.contains("stroke=\"1\""));
        assert!(xml.contains("<contact callsign=\"Route&lt;&amp;\"/>"));
        assert!(xml.contains("callsign=\"Route&lt;&amp; SP\""));
        assert!(xml.contains("callsign=\"VDO\""));
    }

    #[test]
    fn route_links_cover_start_checkpoint_regular_and_end_points() {
        let points = vec![
            RoutePoint { lat: 1.0, lon: 1.0, hae: 1.0 },
            RoutePoint { lat: 2.0, lon: 2.0, hae: 2.0 },
            RoutePoint { lat: 3.0, lon: 3.0, hae: 3.0 },
            RoutePoint { lat: 4.0, lon: 4.0, hae: 4.0 },
            RoutePoint { lat: 5.0, lon: 5.0, hae: 5.0 },
        ];

        let xml = route_links("Route", &points, 2);
        assert!(xml.contains("callsign=\"Route SP\" type=\"b-m-p-w\""));
        assert!(xml.contains("callsign=\"CP1\" type=\"b-m-p-w\""));
        assert!(xml.contains("callsign=\"\" type=\"b-m-p-c\""));
        assert!(xml.contains("callsign=\"VDO\" type=\"b-m-p-w\""));
    }

    #[test]
    fn point_parser_drops_malformed_entries() {
        let points = parse_points("1,2,3;missing;bad,2,3;4,bad,6;7,8,bad;9,10,11");
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].lat, 1.0);
        assert_eq!(points[1].lat, 9.0);
    }
}
