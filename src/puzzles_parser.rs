use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn parse(file: File) {
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        if let Ok(line) = line {
            println!("{}", line);
        }
    }
}
