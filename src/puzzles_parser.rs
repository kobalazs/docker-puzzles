extern crate linked_hash_map;
extern crate yaml_rust;

use self::linked_hash_map::LinkedHashMap;
use self::yaml_rust::{YamlLoader, Yaml};
use fs_handler;

pub fn get_definitions(path: String) -> LinkedHashMap<Yaml, Yaml> {
    let puzzles_paths = fs_handler::collect_files(path, String::from("Puzzles.yml"));
    let mut definitions: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    for puzzles_path in &puzzles_paths {
        let parsed_definitions = parse(fs_handler::read_file(puzzles_path));
        definitions.extend(parsed_definitions);
    }
    definitions
}

fn parse(contents: String) -> LinkedHashMap<Yaml, Yaml> {
    let mut docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = docs.pop().unwrap();

    match doc {
        Yaml::Hash(doc) => return doc,
        _ => panic!("Unsupported Puzzles.yml structure")
    };
}
