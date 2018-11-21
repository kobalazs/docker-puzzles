extern crate docker_puzzles;

use std::fs;
use docker_puzzles::{fs_handler, puzzles_parser};

fn main() {
    let path = String::from("./assets");

    print!("\nPuzzle definitions\n");
    let definitions = puzzles_parser::get_definitions(path);
    print!("{:?}\n", definitions);
    
    print!("\nPuzzlefile\n");
    let puzzles_paths = fs_handler::collect_files(String::from("./assets"), String::from("Puzzlefile"));
    for puzzles_path in &puzzles_paths {
        let content = fs::read_to_string(puzzles_path).expect("Cannot open file");
        print!("{:?}\n", content);
    }
}
