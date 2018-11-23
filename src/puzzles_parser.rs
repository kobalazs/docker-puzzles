extern crate linked_hash_map;
extern crate yaml_rust;

use std::collections::HashMap;
use fs_handler;
use self::yaml_rust::{YamlLoader, Yaml};

pub fn get_puzzles(path: String) -> HashMap<String, String> {
    let puzzles_paths = fs_handler::collect_files(path, String::from("Puzzles.yml"));
    let mut puzzles: HashMap<String, String> = HashMap::new();
    for puzzles_path in &puzzles_paths {
        let parsed_puzzles = parse_puzzles(fs_handler::read_file(puzzles_path));
        puzzles.extend(parsed_puzzles);
    }
    puzzles
}

fn parse_puzzles(contents: String) -> HashMap<String, String> {
    let mut docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = docs.pop().unwrap();
    let entries = match doc {
        Yaml::Hash(doc) => doc,
        _ => panic!("Unsupported Puzzles.yml structure")
    };

    let mut puzzles: HashMap<String, String> = HashMap::new();
    for (key, value) in &entries {
        let key = match key {
            Yaml::String(key) => key.to_string(),
            _ => panic!("Unsupported Puzzles.yml key"),
        };
        let value = match value {
            Yaml::String(value) => value.replace("\\", "\\\n   "),
            _ => panic!("Unsupported Puzzles.yml value"),
        };
        puzzles.insert(key, value);
    }

    puzzles
}
