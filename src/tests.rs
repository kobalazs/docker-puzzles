#![cfg(test)]

use std::process::Command;

static COMMAND: &'static str = "./target/debug/docker-puzzles";

#[test]
fn calling_without_args() {
    let output = Command::new(COMMAND)
        .output()
        .expect("Failed to execute process");

    assert_eq!(
        "Problem parsing arguments: Didn\'t get a directory string\n",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn calling_with_invalid_directory() {
    let output = Command::new(COMMAND)
        .arg("./invalid/directory")
        .output()
        .expect("Failed to execute process");

    assert_eq!(
        "Application error: IO error for operation on ./invalid/directory: No such file or directory (os error 2)\n",
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn calling_with_valid_directory() {
    let output = Command::new(COMMAND)
        .arg("./assets")
        .output()
        .expect("Failed to execute process");

    assert_eq!("", String::from_utf8_lossy(&output.stdout));
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
}
