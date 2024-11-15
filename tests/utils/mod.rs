use std::fs;
use std::io::BufReader;
use std::path::Path;
use newlang::{run, stdio};
use newlang::stdio::STDIN;

pub fn set_testing_stdio_handlers() {
    stdio::set(&STDIN, Box::new(BufReader::new(&[1u8, 2u8, 3u8, 4u8][..])));
}

/// Run a test program, making sure it produces the expected stdout and stderr.
pub fn run_test(path: impl AsRef<Path>) {
    run(&fs::read_to_string(path).unwrap(), None);
}