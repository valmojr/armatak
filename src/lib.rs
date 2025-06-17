use arma_rs::{arma, Extension, Group};
mod structs;
mod tests;
mod udp_socket;
mod tcp_socket;
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
        .group("udp_socket",
            Group::new()
                .command("start", udp_socket::start)
                .command("send_payload", udp_socket::send_payload)
                .command("send_gps_cot", udp_socket::send_gps_cot)
                .command("stop", udp_socket::stop)
        )
        .group(
            "tcp_socket",
            Group::new()
                .command("start", tcp_socket::start)
                .command("send_payload", tcp_socket::send_payload)
                .command("send_eud_cot", tcp_socket::send_eud_cot)
                .command("send_marker_cot", tcp_socket::send_marker_cot)
                .command("send_digital_pointer_cot", tcp_socket::send_digital_pointer_cot)
                .command("stop", tcp_socket::stop)
        )
        .group(
            "video_stream",
            Group::new()
                .command("start", video_stream::start_stream)
                .command("stop", video_stream::stop_stream)
        )
        .finish()
}
