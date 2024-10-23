use clap::Parser;
use std::process;

use doom_colormap_generator as Crate;

fn main() {
    let input = Crate::Input::parse();

    let config = Crate::config_from_input(&input).unwrap_or_else(|err| {
        eprintln!("Error retrieving config: {}", err);
        process::exit(1);
    });

    if let Err(err) = Crate::run(input, config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
