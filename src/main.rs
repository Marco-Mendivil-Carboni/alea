mod params;
mod utils;

use crate::params::MdlPar;
use crate::utils::regex_count;
use anyhow::Result;
use std::fs;
use std::{env, path::Path};

fn main() -> Result<()> {
    utils::init_logger();

    let args: Vec<String> = env::args().collect();

    let name = &args[0];

    println!("Program name: {name}");

    log::info!("This is an info message.");
    log::warn!("This is a warning message.");
    log::error!("This is an error message.");

    let dir = Path::new(&args[1]);

    match regex_count(&dir, "^Cargo.*$") {
        Ok(count) => log::info!("count = {count}"),
        Err(err) => log::error!("{:#?}", err),
    }

    let data = fs::read_to_string("config.json")?;
    let config: MdlPar = serde_json::from_str(&data)?;

    log::info!("matrix = {:#?}", config);

    Ok(())
}
