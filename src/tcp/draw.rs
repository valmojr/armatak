use arma_rs::Context;
use log::info;

use crate::{cot, tcp::send_payload};

fn day_stale() -> (String, String) {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let stale = (chrono::Utc::now() + chrono::Duration::days(1))
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    (now, stale)
}

fn payload_stale(stale_seconds: i64) -> (String, String) {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let stale = (chrono::Utc::now() + chrono::Duration::seconds(stale_seconds.max(1)))
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    (now, stale)
}

pub fn send_circle_cot(
    ctx: Context,
    circle_payload: cot::draws::circle::CircleCoTPayload,
) -> &'static str {
    let shape_circle_cot = circle_payload.to_cot();
    let (now, stale) = day_stale();
    let payload = shape_circle_cot.to_xml(&now, &stale);
    send_payload(ctx, payload);

    "Sending Circle CoT to TCP server"
}

pub fn send_ellipse_cot(
    ctx: Context,
    ellipse_payload: cot::draws::shape::DrawEllipsePayload,
) -> &'static str {
    let (now, stale) = payload_stale(ellipse_payload.stale_seconds);
    let payload = ellipse_payload.to_xml(&now, &stale);
    send_payload(ctx, payload);

    "Sending Ellipse CoT to TCP server"
}

pub fn send_rectangle_cot(
    ctx: Context,
    rectangle_payload: cot::draws::shape::DrawLinksPayload,
) -> &'static str {
    let (now, stale) = payload_stale(rectangle_payload.stale_seconds);
    let payload = rectangle_payload.to_xml(&now, &stale);
    send_payload(ctx, payload);

    "Sending Rectangle CoT to TCP server"
}

pub fn send_freedraw_cot(
    ctx: Context,
    freedraw_payload: cot::draws::shape::DrawLinksPayload,
) -> &'static str {
    let (now, stale) = payload_stale(freedraw_payload.stale_seconds);
    let payload = freedraw_payload.to_xml(&now, &stale);
    send_payload(ctx, payload);

    "Sending Free Draw CoT to TCP server"
}

pub fn send_vectordraw_cot(
    ctx: Context,
    vector_payload: cot::draws::shape::DrawLinksPayload,
) -> &'static str {
    let (now, stale) = payload_stale(vector_payload.stale_seconds);
    let payload = vector_payload.to_xml(&now, &stale);
    send_payload(ctx, payload);

    "Sending Tactical Graphic CoT to TCP server"
}

pub fn send_route_cot(
    ctx: Context,
    route_payload: cot::draws::route::RoutePayload,
) -> &'static str {
    let (now, stale) = payload_stale(route_payload.stale_seconds);
    let payload = route_payload.to_xml(&now, &stale);
    info!(
        "Sending ATAK route '{}' ({} bytes)",
        route_payload.callsign,
        payload.len()
    );
    send_payload(ctx, payload);

    "Sending Route CoT to TCP server"
}
