use arma_rs::Context;

use crate::{cot, tcp::send_payload};

pub fn send_eud_cot(ctx: Context, cursor_over_time: cot::eud::EudCoTPayload) -> &'static str {
    let payload = cursor_over_time.to_cot().convert_to_xml();
    send_payload(ctx, payload);

    "Sending End User Device Cursor Over Time to TCP server"
}

pub fn send_marker_cot(ctx: Context, cursor_over_time: cot::nato::MarkerCoTPayload) -> &'static str {
    let payload = cursor_over_time.to_cot().convert_to_xml();
    send_payload(ctx, payload);

    "Sending Marker Cursor Over Time to TCP server"
}

pub fn send_digital_pointer_cot(ctx: Context, cursor_over_time: cot::digital_pointer::DigitalPointerPayload) -> &'static str {
    let payload = cursor_over_time.to_cot().convert_to_xml();
    send_payload(ctx, payload);

    "Sending Digital Pointer Cursor Over Time to TCP server"
}

pub fn send_message_cot(ctx: Context, message_payload: cot::message::MessagePayload) -> &'static str {
    let message_cot = cot::message::MessageCot::from_payload(message_payload);
    let payload = message_cot.to_xml();
    send_payload(ctx, payload);

    "Sending Message CoT to TCP server"
}
