use chrono::Local;
use colored::Colorize;
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init_logger() {
    Builder::new()
        .format(|buffer, record| {
            let timestamp = Local::now().format("%d/%m/%y %H:%M:%S%.6f");
            let level_tag = format!("{:5}", record.level().as_str());
            let level_tag = match record.level() {
                Level::Error => level_tag.red().bold(),
                Level::Warn => level_tag.yellow(),
                _ => level_tag.green(),
            };
            writeln!(buffer, "{} [{}] {}", timestamp, level_tag, record.args())
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
}
