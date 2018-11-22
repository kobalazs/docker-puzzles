extern crate docker_puzzles;

use docker_puzzles::{dockerfile_builder, fs_handler, puzzles_parser};

fn main() {
    let path = String::from("./assets");
    let puzzles = puzzles_parser::get_puzzles(path);

    let puzzlefiles = fs_handler::collect_files(String::from("./assets"), String::from("Puzzlefile"));
    for puzzlefile in &puzzlefiles {
        dockerfile_builder::build(puzzlefile, &puzzles);
    }
}
