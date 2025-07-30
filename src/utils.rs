use anyhow::{Context, Result};
use chrono::Local;
use colored::Colorize;
use env_logger::Builder;
use log::{Level, LevelFilter};
use regex::Regex;
use std::{fs, io::Write, path::Path};

pub fn init_logger() {
    Builder::new()
        .format(|buffer, record| {
            let timestamp = Local::now().format("%d/%m/%y %H:%M:%S%.6f");

            let level_tag = record.level().as_str().to_lowercase();
            let level_tag = match record.level() {
                Level::Error => level_tag.red().bold(),
                Level::Warn => level_tag.yellow(),
                _ => level_tag.green(),
            };

            let file = record.file().unwrap_or("unknown");
            let line = record
                .line()
                .map(|l| l.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            writeln!(
                buffer,
                "{} [{:5}] [{}:{}] {}",
                timestamp,
                level_tag,
                file,
                line,
                record.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
}

// count directory entries matching regex pattern
pub fn regex_count(dir: &Path, pattern: &str) -> Result<usize> {
    let regex = Regex::new(pattern)?;

    let count = fs::read_dir(dir)
        .with_context(|| format!("Failed to read {}", dir.display()))?
        .filter_map(Result::ok)
        .filter(|entry| match entry.file_name().to_str() {
            Some(name) if regex.is_match(name) => true,
            _ => false,
        })
        .count();

    Ok(count)
}
