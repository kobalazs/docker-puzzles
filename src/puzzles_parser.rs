use crate::error::UserError;
use crate::fs_handler;
use std::collections::HashMap;
use std::error::Error;
use yaml_rust::{Yaml, YamlLoader};

pub fn get_puzzles(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let puzzles_paths = fs_handler::collect_files(path, "Puzzles.yml")?;

    let mut puzzles: HashMap<String, String> = HashMap::new();
    for puzzles_path in &puzzles_paths {
        let puzzles_content = fs_handler::read_file(puzzles_path)?;
        let parsed_puzzles = parse_puzzles(puzzles_content.as_str())?;
        puzzles.extend(parsed_puzzles);
    }

    Ok(puzzles)
}

fn parse_puzzles(contents: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut docs = YamlLoader::load_from_str(&contents)?;
    let doc = docs.pop().unwrap();
    let entries = match doc {
        Yaml::Hash(doc) => doc,
        _ => return Err(UserError::boxed("Unsupported Puzzles.yml structure")),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_puzzles() {
        let puzzles = get_puzzles("./assets").unwrap();
        let key = String::from("echos");
        let value = String::from("RUN echo \'a\' \\\n    && echo \'b\'");
        assert_eq!(
            [(key, value)]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>(),
            puzzles
        );
    }
}
