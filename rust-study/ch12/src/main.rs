use std::{env, process};

// use minigrep::Config;

mod lib;

// use crate::lib::minigrep::Config;
// use crate::lib::minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let config = lib::minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
