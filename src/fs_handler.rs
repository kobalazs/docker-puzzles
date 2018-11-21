use std::fs;
use std::path::{Path, PathBuf};

pub fn collect_files(path: String, file_name: String) -> Vec<PathBuf> {
    let path = Path::new(&path);
    let mut file_paths = Vec::new();
    for entry in path.read_dir().expect("Cannot open directory") {
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

pub fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("Cannot open file")
}
