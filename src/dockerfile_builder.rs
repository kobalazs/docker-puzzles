extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use self::regex::Regex;

pub fn build(puzzlefile: &Path, puzzles: &HashMap<String, String>) {
    let puzzlefile = File::open(puzzlefile).expect("Unable to open Puzzlefile");
    let mut dockerfile = File::create("./assets/Dockerfile").expect("Unable to open Dockerfile");

    for line in BufReader::new(puzzlefile).lines() {
        let input = line.unwrap();
        let output = parse_line(input, puzzles) + "\n";
        dockerfile.write(output.as_bytes()).expect("Unable to write Dockerfile");
    }
}

fn parse_line(input: String, puzzles: &HashMap<String, String>) -> String {
    let regex = Regex::new(r"^PUZZLE (.+)").unwrap();
    if regex.is_match(&input) {
        let captures = regex.captures(&input).unwrap();
        return puzzles.get(&captures[1]).expect("Undefined puzzle").to_string();
    }
    input
}
