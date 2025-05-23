use arma_rs::{arma, Extension, Group};
mod structs;
mod tests;
mod websocket;
mod cot_router;
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
        .group("websocket", Group::new()
            .command("start", websocket::start)
            .command("stop", websocket::stop)
            .command("message", websocket::message)
            .command("location", websocket::location)    
        )
        .command("local_ip", utils::address::get_local_address)
        .command("uuid", utils::uuid::get_uuid)
        .command("log", utils::log::log_info)
        .group(
            "tcp_socket",
            Group::new()
                .command("start", cot_router::start)
                .command("send_payload", cot_router::send_payload)
                .command("send_eud_cot", cot_router::send_eud_cot)
                .command("send_marker_cot", cot_router::send_marker_cot)
                .command("send_digital_pointer_cot", cot_router::send_digital_pointer_cot)
                .command("stop", cot_router::stop)
        )
        .group(
            "video_stream",
            Group::new()
                .command("start", video_stream::start_stream)
                .command("stop", video_stream::stop_stream)
        )
        .finish()
}
