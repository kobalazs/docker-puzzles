extern crate docker_puzzles;

use std::fs;
use docker_puzzles::{fs_handler, puzzles_parser};

fn main() {
    let path = String::from("./assets");

    let puzzles = puzzles_parser::get_puzzles(path);
    print!("\n[php-libs]\n{:?}\n", puzzles.get("php-libs"));
    
    print!("\nPuzzlefile\n");
    let puzzlefiles = fs_handler::collect_files(String::from("./assets"), String::from("Puzzlefile"));
    for puzzlefile in &puzzlefiles {
        let content = fs::read_to_string(puzzlefile).expect("Cannot open file");
        print!("{:?}\n", content);
    }
}
