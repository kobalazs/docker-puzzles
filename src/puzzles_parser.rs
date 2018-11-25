extern crate linked_hash_map;
extern crate yaml_rust;

use std::collections::HashMap;
use std::error::Error;
use self::yaml_rust::{YamlLoader, Yaml};
use fs_handler;
use error::UserError;

pub fn get_puzzles(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let puzzles_paths = fs_handler::collect_files(path, String::from("Puzzles.yml"))?;

    let mut puzzles: HashMap<String, String> = HashMap::new();
    for puzzles_path in &puzzles_paths {
        let parsed_puzzles = parse_puzzles(fs_handler::read_file(puzzles_path))?;
        puzzles.extend(parsed_puzzles);
    }

    Ok(puzzles)
}

fn parse_puzzles(contents: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut docs = YamlLoader::load_from_str(&contents)?;
    let doc = docs.pop().unwrap();
    let entries = match doc {
        Yaml::Hash(doc) => doc,
        _ => return Err(UserError::boxed("Unsupported Puzzles.yml structure"))
    };

    let mut puzzles: HashMap<String, String> = HashMap::new();
    for (key, value) in &entries {
        let key = match key {
            Yaml::String(key) => key.to_string(),
            _ => return Err(UserError::boxed("Unsupported Puzzles.yml key")),
        };
        let value = match value {
            Yaml::String(value) => value.replace("\\", "\\\n   "),
            _ => return Err(UserError::boxed("Unsupported Puzzles.yml value")),
        };
        puzzles.insert(key, value);
    }

    Ok(puzzles)
}
