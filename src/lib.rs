use arma_rs::{arma, Extension, Group};
mod structs;
mod tcp;
mod tests;
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

    Extension::build()
        .command("local_ip", utils::address::get_local_address)
        .command("uuid", utils::uuid::get_uuid)
        .command("log", utils::log::log_info)
        .group(
            "udp_socket",
            Group::new()
                .command("start", udp_socket::start)
                .command("send_payload", udp_socket::send_payload)
                .command("send_gps_cot", udp_socket::send_gps_cot)
                .command("stop", udp_socket::stop),
        )
        .group(
            "tcp_socket",
            Group::new()
                .command("start", tcp::start)
                .command("stop", tcp::stop)
                .command("send_payload", tcp::send_payload)
                .group(
                    "cot",
                    Group::new()
                        .command("eud", tcp::cot::send_eud_cot)
                        .command("marker", tcp::cot::send_marker_cot)
                        .command("digital_pointer", tcp::cot::send_digital_pointer_cot)
                        .command("chat", tcp::cot::send_message_cot),
                )
                .group(
                    "draw",
                    Group::new()
                        .command("circle", tcp::draw::send_circle_cot)
                        .command("ellipse", tcp::draw::send_ellipse_cot)
                        .command("rectangle", tcp::draw::send_rectangle_cot)
                        .command("free", tcp::draw::send_freedraw_cot)
                        .command("vector", tcp::draw::send_vectordraw_cot),
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
