use arma_rs::{arma, Extension, Group};
use rustls::crypto::aws_lc_rs;
mod mdns;
mod structs;
mod tcp;
mod tests;
mod uas;
mod udp_socket;
mod video_stream;

mod cot;
mod utils;

#[arma]
pub fn init() -> Extension {
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let file_appender = FileAppender::builder()
        .append(true)
        .encoder(Box::new(PatternEncoder::new("{d} {t} - {m}{n}")))
        .build("armatak.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(
            Root::builder()
                .appender("file")
                .build(log::LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    let _ = aws_lc_rs::default_provider().install_default();
    log::info!("Initialized rustls aws-lc crypto provider.");

    Extension::build()
        .command("local_ip", utils::address::get_local_address)
        .command("uuid", utils::uuid::get_uuid)
        .command("log", utils::log::log_info)
        .group(
            "uas",
            Group::new()
                .command("start_endpoint", uas::start_endpoint)
                .command("stop_endpoint", uas::stop_endpoint)
                .command("send_uas_telemetry", uas::send_uas_telemetry)
                .command("send_uas_system", uas::send_uas_system),
        )
        .group(
            "mdns",
            Group::new()
                .command("start_uas_advertisement", mdns::start_uas_advertisement)
                .command("stop", mdns::stop),
        )
        .group(
            "udp_socket",
            Group::new()
                .command("start", udp_socket::start)
                .command("start_lrf", udp_socket::start_lrf)
                .command("start_cot", udp_socket::start_cot)
                .command("send_payload", udp_socket::send_payload)
                .command("send_gps_cot", udp_socket::send_gps_cot)
                .command("send_eud_cot", udp_socket::send_eud_cot)
                .command("send_lrf", udp_socket::send_lrf)
                .command("clear_lrf", udp_socket::clear_lrf)
                .command(
                    "send_digital_pointer_cot",
                    udp_socket::send_digital_pointer_cot,
                )
                .command("stop", udp_socket::stop),
        )
        .group(
            "tcp_socket",
            Group::new()
                .command("start", tcp::start)
                .command("start_mtls", tcp::start_mtls)
                .command("start_enroll_mtls", tcp::start_enroll_mtls)
                .command("stop", tcp::stop)
                .command("send_payload", tcp::send_payload)
                .group(
                    "cot",
                    Group::new()
                        .command("eud", tcp::cot::send_eud_cot)
                        .command("marker", tcp::cot::send_marker_cot)
                        .command("report_marker", tcp::cot::send_report_marker_cot)
                        .command("digital_pointer", tcp::cot::send_digital_pointer_cot)
                        .command("delete", tcp::cot::send_delete_cot)
                        .command("chat", tcp::cot::send_message_cot)
                        .command("uas_platform", tcp::cot::send_uas_platform_cot)
                        .command("uas_video", tcp::cot::send_uas_video_cot)
                        .command("uas_sensor", tcp::cot::send_uas_sensor_cot),
                )
                .group(
                    "draw",
                    Group::new()
                        .command("circle", tcp::draw::send_circle_cot)
                        .command("ellipse", tcp::draw::send_ellipse_cot)
                        .command("rectangle", tcp::draw::send_rectangle_cot)
                        .command("free", tcp::draw::send_freedraw_cot)
                        .command("vector", tcp::draw::send_vectordraw_cot)
                        .command("route", tcp::draw::send_route_cot),
                ),
        )
        .group(
            "video_stream",
            Group::new()
                .command("start", video_stream::start_stream)
                .command("stop", video_stream::stop_stream),
        )
        .finish()
}
