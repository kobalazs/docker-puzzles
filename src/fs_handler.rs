extern crate walkdir;

use std::fs;
use std::path::Path;
use std::error::Error;
use self::walkdir::WalkDir;

pub fn collect_files(path: &str, file_name: String) -> Result<Vec<&Path>, Box<dyn Error>> {
    let mut file_paths = Vec::new();
    for entry in WalkDir::new(path) {
        // println!("{}", entry?.path().display());
        let actual_path = match_file(entry?.path(), &file_name);
        if let Some(actual_path) = actual_path {
            file_paths.push(actual_path.clone());
        }
    }

    // let path = Path::new(&path);
    // let directory = path.read_dir()?;
    // let mut file_paths = Vec::new();
    // for entry in directory {
    //     if let Ok(entry) = entry {
    //         let actual_path = entry.path();
    //         let actual_file_name = actual_path.file_name();
    //         if let Some(actual_file_name) = actual_file_name {
    //             if actual_file_name.to_str().unwrap() == &file_name {
    //                 file_paths.push(actual_path.clone());
    //             }
    //         }
    //     }
    // }
    Ok(file_paths)
}

fn match_file<'a>(path: &Path, file_name: &String) -> Option<&'a Path> {
    let actual_file_name = path.file_name();
    if let Some(actual_file_name) = actual_file_name {
        if actual_file_name.to_str().unwrap() == file_name {
            return Some<'a: Path>(path);
        }
    }
    None
}

pub fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Cannot open file")
}
