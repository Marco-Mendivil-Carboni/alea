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

// count files matching pattern
pub fn regex_count(dir: &Path, pattern: &str) -> Result<usize> {
    let regex = Regex::new(pattern)?;
    let mut count = 0;
    for entry in fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory '{}'", dir.display()))?
    {
        let entry = entry?;
        if let Some(file_name_str) = entry.file_name().to_str() {
            if regex.is_match(file_name_str) {
                count += 1;
            }
        }
    }
    Ok(count)
}
