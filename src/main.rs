use std::env;
use std::process;

use doom_colormap_generator as colorgen;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = colorgen::input_from_args(&args)
        .unwrap_or_else(|err| {
            eprintln!("Invalid arguments: {}", err);
            process::exit(1);
        });
    
    let config = colorgen::config_from_input(input)
        .unwrap_or_else(|err| {
            eprintln!("Error retrieving config: {}", err);
            process::exit(1);
        });
    
    if let Err(err) = colorgen::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
