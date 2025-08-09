mod data;
mod engine;
mod params;
mod utils;

use crate::engine::SimEng;
use crate::params::Params;
use anyhow::Context;
use anyhow::Result;
use ron::ser::{PrettyConfig, to_string_pretty};
use std::env;
use std::fs;

fn main() -> Result<()> {
    utils::init_logger();

    let args: Vec<String> = env::args().collect();

    let name = &args[0];

    log::info!("program name = {name}");

    match utils::count_entries(&args[1], "^Cargo.*$") {
        Ok(count) => log::info!("count = {count}"),
        Err(err) => log::error!("{:#}", err),
    }

    let par_str = fs::read_to_string("parameters.ron")?;
    let par = Params::new(&par_str)
        .context("failed to create parameters")
        .unwrap_or_else(|err| {
            log::error!("{:?}", err);
            std::process::exit(1);
        });

    log::info!("par = {:#?}", par);

    let par_str = to_string_pretty(&par, PrettyConfig::default())?;
    fs::write("parameters.ron", par_str)?;

    let mut sim_eng = SimEng::new(par)?;
    sim_eng.generate_initial_condition()?;
    sim_eng.run_simulation("frame.bin")?;

    Ok(())
}
