use arma_rs::{arma, Extension, Group};
mod structs;
mod tests;
mod websocket;
mod util;
mod cot_router;
mod cot_generator;

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

    websocket::start();

    Extension::build()
        .group("websocket", Group::new()
            .command("message", websocket::message)
            .command("location", websocket::location)    
        )
        .command("local_ip", util::get_local_address)
        .command("uuid", util::get_uuid)
        .command("callback", util::test_callback)
        .command("log", util::log_info)
        .group(
            "cot_router",
            Group::new()
                .command("start", cot_router::start)
                .command("send_payload", cot_router::send_payload)
                .command("send_human_cot", cot_router::send_human_cot)
                .command("send_marker_cot", cot_router::send_marker_cot)
                .command("stop", cot_router::stop)
        )
        .finish()
}
