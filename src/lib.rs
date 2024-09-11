use arma_rs::{arma, Extension, Group};
use util::get_uuid;
mod commands;
mod services;
mod structs;
mod tests;
mod util;
#[arma]
pub fn init() -> Extension {
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let file_appender = FileAppender::builder()
        .append(true)
        .encoder(Box::new(PatternEncoder::new("{d} {t} {l} - {m}{n}")))
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
        .command("uuid", get_uuid)
        .group(
            "markers",
            Group::new()
                .command("get", commands::markers::get)
                .command("post", commands::markers::post)
                .command("delete", commands::markers::delete),
        )
        .group(
            "casevac",
            Group::new()
                .command("get", commands::casevac::get)
                .command("post", commands::casevac::post)
                .command("delete", commands::casevac::delete),
        )
        .finish()
}
