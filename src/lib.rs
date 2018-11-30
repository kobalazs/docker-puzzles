pub mod config;
pub mod dockerfile_builder;
pub mod error;
pub mod fs_handler;
pub mod puzzles_parser;
pub mod tests;

use std::error::Error;
use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let puzzlefiles = fs_handler::collect_files(&config.directory, "Puzzlefile")?;
    let puzzles = puzzles_parser::get_puzzles(&config.directory)?;
    
    for puzzlefile in &puzzlefiles {
        dockerfile_builder::build(puzzlefile, &puzzles)?;
    }

    Ok(())
}
