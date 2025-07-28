use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_logger() {
    Builder::new()
        .format(|buf, record| {
            let timestamp = Local::now().format("%d/%m/%y %H:%M:%S");
            writeln!(buf, "{} [{}] {}", timestamp, record.level(), record.args())
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
}
