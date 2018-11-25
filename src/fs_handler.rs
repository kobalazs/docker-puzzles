use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

pub fn collect_files(path: &str, file_name: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let path = Path::new(&path);
    let directory = path.read_dir()?;

    let mut file_paths = Vec::new();
    for entry in directory {
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
    Ok(file_paths)
}

pub fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("Cannot open file")
}
