use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path = Path::new("./assets");
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = path.file_name();
            if let Some(file_name) = file_name {
                if file_name == "Puzzles" {
                    let input = File::open(path.clone()).expect("Cannot open file");
                    let buffered = BufReader::new(input);

                    for line in buffered.lines() {
                        if let Ok(line) = line {
                            println!("{}", line);
                        }
                    }
                }
            }
        }
    }
}
