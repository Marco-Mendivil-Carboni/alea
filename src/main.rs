mod utils;

use anyhow::Result;
use std::{env, path::Path};

use crate::utils::regex_count;

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

    Ok(())
}
