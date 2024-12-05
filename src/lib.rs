use arma_rs::{arma, Extension, Group};
mod commands;
mod structs;
mod tests;
mod websocket;
mod api;
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

    Extension::build()
        .group("api", Group::new()
            .command("start", websocket::start)
            .command("stop", websocket::stop)
            .command("message", websocket::message)
            .command("location", websocket::location)    
        )
        .command("local_ip", util::get_local_address)
        .command("uuid", util::get_uuid)
        .command("get_auth_token", api::get_auth_token)
        .group(
            "markers",
            Group::new()
                .command("get", api::markers::get)
                .command("post", api::markers::post)
                .command("post_debug", api::markers::post_debug)
                .command("delete", api::markers::delete),
        )
        .finish()
}
