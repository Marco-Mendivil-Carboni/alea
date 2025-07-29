mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = &args[0];

    println!("Program name: {name}");

    utils::init_logger();

    log::trace!("...");
    log::debug!("hey");
    log::info!("This is an info message.");
    log::warn!("This is a warning message.");
    log::error!("This is an error message.");
}
