mod utils;

use crate::utils::regex_count;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::{env, path::Path};

#[derive(Deserialize)]
struct Config {
    matrix: Vec<Vec<f64>>,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let name = &args[0];

    println!("Program name: {name}");

    utils::init_logger();

    log::trace!("...");
    log::debug!("hey");
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");

    let dir = Path::new(&args[1]);

    match regex_count(&dir, "^Cargo.*$") {
        Ok(count) => log::info!("count = {count}"),
        Err(err) => log::error!("{:#}", err),
    }

    let data = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&data)?;

    log::info!("matrix = {:?}", config.matrix);

    Ok(())
}
