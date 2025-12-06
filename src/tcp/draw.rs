use arma_rs::Context;

use crate::{cot, tcp::send_payload};

pub fn send_circle_cot(ctx: Context, circle_payload: cot::draws::circle::CircleCoTPayload) -> &'static str {
    let shape_circle_cot = circle_payload.to_cot();
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let stale = (chrono::Utc::now() + chrono::Duration::days(1))
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let payload = shape_circle_cot.to_xml(&now, &stale);
    send_payload(ctx, payload);

    "Sending Circle CoT to TCP server"
}

pub fn send_ellipse_cot(ctx: Context) -> &'static str {
  let _ = ctx;
  "Not implemented: send_ellipse_cot"
}

pub fn send_rectangle_cot(ctx: Context) -> &'static str {
  let _ = ctx;
  "Not implemented: send_ellipse_cot"
}

pub fn send_freedraw_cot(ctx: Context) -> &'static str {
  let _ = ctx;
  "Not implemented: send_ellipse_cot"
}

pub fn send_vectordraw_cot(ctx: Context) -> &'static str {
  let _ = ctx;
  "Not implemented: send_ellipse_cot"
}