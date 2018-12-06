use docker_puzzles::{config::Config, run};
use std::{env, process};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(error) = run(&config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}
