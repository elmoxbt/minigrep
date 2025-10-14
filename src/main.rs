extern crate rust_cli;

use std::env;
use std::process;

use rust_cli::Config;



fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file '{}'", config.query, config.filename);

    if let Err(e) = rust_cli::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }


}

