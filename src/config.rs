use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

fn init_log() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(format!(
            "log/log_{}.log",
            chrono::Utc::now().timestamp_millis()
        ))
        .unwrap();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                // .appender("file")
                .build(LevelFilter::Info),
        )
        .unwrap();

    let result = log4rs::init_config(config);
    if result.is_err() {
        print!("init log error : {:?}", result);
    }
}

pub fn init_config() {
    // let r = log4rs::init_file("config/log4rs.yaml", Default::default());

    // if r.is_err() {
    //     let _ = log4rs::init_file("rust/config/log4rs.yaml", Default::default());
    // }
    init_log();
}
