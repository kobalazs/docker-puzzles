extern crate linked_hash_map;
extern crate yaml_rust;

use self::linked_hash_map::LinkedHashMap;
use self::yaml_rust::{YamlLoader, Yaml};

pub fn parse(contents: String) -> LinkedHashMap<Yaml, Yaml> {
    let mut docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = docs.pop().unwrap();

    match doc {
        Yaml::Hash(doc) => return doc,
        _ => panic!("Unsupported Puzzles.yml structure!")
    };
}
