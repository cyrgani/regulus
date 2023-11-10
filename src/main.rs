use atxlang::*;
use std::fs;

fn main() {
    test_prog();
}

/// Reads a file and returns the content.
fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| panic!("No file {file_path} was found!"))
}

fn test_prog() {
    run(&read_file("./programs/test.prog")).unwrap();
}
