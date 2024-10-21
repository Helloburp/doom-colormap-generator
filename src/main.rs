use std::process;
use clap::Parser;

use doom_colormap_generator as colorgen;

fn main() {
    let input = colorgen::Input::parse();
    
    let config = colorgen::config_from_input(&input)
        .unwrap_or_else(|err| {
            eprintln!("Error retrieving config: {}", err);
            process::exit(1);
        });
    
    if let Err(err) = colorgen::run(input, config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
