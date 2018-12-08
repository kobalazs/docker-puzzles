use crate::error::UserError;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn build<S: ::std::hash::BuildHasher>(
    puzzlefile_path: &Path,
    puzzles: &HashMap<String, String, S>,
) -> Result<(), Box<dyn Error>> {
    let puzzlefile = File::open(puzzlefile_path)?;
    let mut dockerfile = File::create(get_dockerfile_path(puzzlefile_path)?)?;

    for line in BufReader::new(puzzlefile).lines() {
        let input = line.unwrap();
        let output = parse_line(input, puzzles)? + "\n";
        dockerfile.write_all(output.as_bytes())?;
    }

    Ok(())
}

fn get_dockerfile_path(puzzlefile_path: &Path) -> Result<String, Box<dyn Error>> {
    if let Some(parent_path) = puzzlefile_path.parent() {
        if let Some(parent_path) = parent_path.to_str() {
            return Ok(parent_path.to_owned() + "/Dockerfile");
        }
    }
    Err(UserError::boxed("Cannot find parent directory"))
}

fn parse_line<S: ::std::hash::BuildHasher>(
    input: String,
    puzzles: &HashMap<String, String, S>,
) -> Result<String, Box<dyn Error>> {
    let regex = Regex::new(r"^PUZZLE (.+)").unwrap();
    if regex.is_match(&input) {
        let captures = regex.captures(&input).unwrap();
        return match puzzles.get(&captures[1]) {
            Some(puzzle) => Ok(puzzle.to_string()),
            None => Err(UserError::boxed("Undefined puzzle")),
        };
    }
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    #[test]
    fn it_builds_dockerfile() {
        let path = PathBuf::from("./assets/a/Puzzlefile");
        let mut puzzles: HashMap<String, String> = HashMap::new();
        puzzles.insert("echos".to_string(), "RUN echo 'Hello World'".to_string());

        assert_eq!((), build(&path, &puzzles).unwrap());
        assert_eq!(
            "FROM ubuntu:trusty\n\nRUN echo 'Hello World'\n",
            read_to_string("./assets/a/Dockerfile").unwrap()
        );
    }
}
