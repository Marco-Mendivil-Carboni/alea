mod params;
mod utils;

use crate::params::MdlPar;
use crate::utils::regex_count;
use anyhow::Result;
use std::env;
use std::fs;

fn main() -> Result<()> {
    utils::init_logger();

    let args: Vec<String> = env::args().collect();

    let name = &args[0];

    println!("Program name: {name}");

    log::info!("info message");
    log::warn!("warning message");
    log::error!("error message");

    match regex_count(&args[1], "^Cargo.*$") {
        Ok(count) => log::info!("count = {count}"),
        Err(err) => log::error!("{:#?}", err),
    }

    let json_str = fs::read_to_string("config.json")?;
    let json_val: serde_json::Value = serde_json::from_str(&json_str)?;
    let mdlpar = MdlPar::new(json_val).unwrap_or_else(|err| {
        log::error!("Failed to initialize model parameters: {:#?}", err);
        std::process::exit(1);
    });

    log::info!("matrix = {:#?}", mdlpar);

    Ok(())
}
