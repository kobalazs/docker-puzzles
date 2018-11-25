extern crate docker_puzzles;

use std::{env, process};
use docker_puzzles::{config::Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(error) = docker_puzzles::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}
