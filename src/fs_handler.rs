extern crate walkdir;

use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use self::walkdir::WalkDir;

pub fn collect_files(path: &str, file_name: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_collects_all_files() {
        let file_paths = collect_files("./assets", "Puzzlefile").unwrap();
        assert_eq!(2, file_paths.len());
        assert_eq!("./assets/a/Puzzlefile", file_paths.get(0).unwrap().to_str().unwrap());
        assert_eq!("./assets/b/Puzzlefile", file_paths.get(1).unwrap().to_str().unwrap());
    }


    #[test]
    fn it_returns_error_when_called_on_missing_directory() {
        let error = collect_files("./non-existent-dir", "Puzzlefile").expect_err("Error expected");
        assert_eq!(
            "IO error for operation on ./non-existent-dir: No such file or directory (os error 2)",
            error.to_string()
        );
    }

    #[test]
    fn it_reads_a_file() {
        let contents = read_file(&Path::new("./assets/Puzzles.yml")).unwrap();
        assert_eq!("echos:\n    RUN echo \'a\' \\\n        && echo \'b\'\n", contents);
    }

    #[test]
    fn it_returns_error_when_called_on_missing_file() {
        let error = read_file(&Path::new("./non-existent-file")).expect_err("Error expected");
        assert_eq!(
            "No such file or directory (os error 2)",
            error.to_string()
        );
    }
}
