use arma_rs::{arma, Extension, Group};
mod structs;
mod tests;
mod websocket;
mod ots_api;
mod fts_api;
mod util;

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
        .command("log", util::log_info)
        .group(
            "ots_api",
            Group::new()
                .command("get", ots_api::markers::get)
                .command("get_auth_token", ots_api::get_auth_token)
                .command("post", ots_api::markers::post)
                .command("post_debug", ots_api::markers::post_debug)
                .command("delete", ots_api::markers::delete),
        )
        .group(
            "fts_api",
            Group::new()
                .command("get", fts_api::markers::get)
                .command("get_all", fts_api::markers::get_all)
                .command("post", fts_api::markers::post)
                .command("patch", fts_api::markers::patch)
                .command("delete", fts_api::markers::delete)
        )
        .finish()
}
