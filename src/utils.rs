use anyhow::{Context, Result};
use env_logger::Builder;
use log::LevelFilter;
use regex::Regex;
use std::{fs, path::Path};

pub fn init_logger() {
    Builder::new()
        .format_timestamp_millis()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
}

pub fn count_entries<P: AsRef<Path>>(dir: P, regex: &str) -> Result<usize> {
    let dir = dir.as_ref();
    let regex = Regex::new(regex)?;
    let count = fs::read_dir(dir)
        .with_context(|| format!("failed to read {:?}", dir))?
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
