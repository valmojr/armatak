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
