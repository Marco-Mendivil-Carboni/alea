mod data;
mod engine;
mod params;
mod utils;

use crate::data::{AgtData, SimData};
use crate::engine::SimEng;
use crate::params::Params;
use crate::utils::regex_count;
use anyhow::Context;
use anyhow::Result;
use ndarray::Array1;
use ron::ser::{PrettyConfig, to_string_pretty};
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    utils::init_logger();

    let args: Vec<String> = env::args().collect();

    let name = &args[0];

    log::info!("program name = {name}");

    match regex_count(&args[1], "^Cargo.*$") {
        Ok(count) => log::info!("count = {count}"),
        Err(err) => log::error!("{:#}", err),
    }

    let par_str = fs::read_to_string("parameters.ron")?;
    let par = Params::new(&par_str)
        .context("failed to initialize parameters")
        .unwrap_or_else(|err| {
            log::error!("{:?}", err);
            std::process::exit(1);
        });

    log::info!("par = {:#?}", par);

    let par_str = to_string_pretty(&par, PrettyConfig::default())?;
    fs::write("parameters.ron", par_str)?;

    let path = Path::new("frame.bin");
    let mut sim_data = if path.exists() {
        SimData::load_from_file(path)?
    } else {
        SimData {
            env: 0,
            agt_vec: Vec::new(),
            n_agt_diff: 0,
        }
    };

    log::info!("SimData loaded successfully: {:?}", sim_data);

    let n_phe = par.n_phe;
    let phe = 0;
    let prob_phe = Array1::from(vec![1.0 / par.n_phe as f64; par.n_phe]);
    let agt = AgtData::new(phe, prob_phe, n_phe).context("failed to create new agent")?;
    sim_data.agt_vec.push(agt);

    sim_data.save_to_file("frame.bin")?;

    Ok(())
}
