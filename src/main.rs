extern crate docker_puzzles;

use std::path::{Path, PathBuf};
use std::fs;
use docker_puzzles::puzzles_parser::parse;

fn main() {
    print!("\nPuzzles.yml\n");
    let puzzles_paths = collect_files(String::from("./assets"), String::from("Puzzles.yml"));
    for puzzles_path in &puzzles_paths {
        let content = fs::read_to_string(puzzles_path).expect("Cannot open file");
        print!("{:?}\n", parse(content));
    }

    print!("\nPuzzlefile\n");
    let puzzles_paths = collect_files(String::from("./assets"), String::from("Puzzlefile"));
    for puzzles_path in &puzzles_paths {
        let content = fs::read_to_string(puzzles_path).expect("Cannot open file");
        print!("{:?}\n", content);
    }
}

fn collect_files(path: String, file_name: String) -> Vec<PathBuf> {
    let path = Path::new(&path);
    let mut file_paths = Vec::new();
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let actual_path = entry.path();
            let actual_file_name = actual_path.file_name();
            if let Some(actual_file_name) = actual_file_name {
                if actual_file_name.to_str().unwrap() == &file_name {
                    file_paths.push(actual_path.clone());
                }
            }
        }
    }
    file_paths
}
