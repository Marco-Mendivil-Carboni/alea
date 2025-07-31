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

    log::info!("program name = {name}");

    match regex_count(&args[1], "^Cargo.*$") {
        Ok(count) => log::info!("count = {count}"),
        Err(err) => log::error!("{:#?}", err),
    }

    let json_str = fs::read_to_string("config.yaml")?;
    let json_val: serde_yaml::Value = serde_yaml::from_str(&json_str)?;
    let mdl_par = MdlPar::new(json_val).unwrap_or_else(|err| {
        log::error!("failed to initialize model parameters: {:#?}", err);
        std::process::exit(1);
    });

    log::info!("mdl_par = {:#?}", mdl_par);

    Ok(())
}
