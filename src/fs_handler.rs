extern crate walkdir;

use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use self::walkdir::WalkDir;

pub fn collect_files(path: &str, file_name: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut file_paths = Vec::new();
    for entry in WalkDir::new(path) {
        let entry_path = entry?.into_path();
        if let Some(actual_file_name) = entry_path.file_name() {
            if actual_file_name.to_str().unwrap() == file_name {
                file_paths.push(entry_path.clone());
            }
        }
    }
    Ok(file_paths)
}

pub fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}
