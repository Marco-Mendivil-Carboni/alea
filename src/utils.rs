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

pub fn regex_count<P>(dir: P, pattern: &str) -> Result<usize>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    let regex = Regex::new(pattern)?;

    let count = fs::read_dir(dir)
        .with_context(|| format!("failed to read {}", dir.display()))?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .is_some_and(|name| regex.is_match(name))
        })
        .count();

    Ok(count)
}

fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}
