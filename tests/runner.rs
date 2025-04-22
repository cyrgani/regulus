use regulus::prelude::State;
use regulus::{FILE_EXTENSION, Runner};
use std::fs;
use std::path::{Path, PathBuf};
use std::str;
use tests::make_tests;

fn read_file_or_empty(base_path: &Path, extension: &str) -> String {
    fs::read_to_string(base_path.with_extension(extension)).unwrap_or_default()
}

fn write_file_if_nonempty(base_path: &Path, extension: &str, content: &str) {
    if content.is_empty() {
        return;
    }
    fs::write(base_path.with_extension(extension), content).unwrap();
}

/// Run a test program, making sure it produces the expected stdout and stderr.
pub fn run_test(dir_path: &str, name: &str) {
    //let mut overwrite_stream_files = env::args().any(|arg| arg == "--bless");
    let mut bless_stream_files = false;
    if let Some(var) = BLESS {
        if ["y", "yes", "true"].contains(&var) {
            bless_stream_files = true;
        }
    }

    let base_path = PathBuf::from(dir_path).join(name);

    let (res, final_state) = Runner::new()
        .file(base_path.with_extension(FILE_EXTENSION))
        .expect("fatal error: program file not found")
        .stl_dir("../stdlib")
        .starting_state(State::testing_setup(
            dir_path,
            &read_file_or_empty(&base_path, "stdin"),
        ))
        .run();

    let stdout = final_state.testing_read_stdout();
    let mut stderr = final_state.testing_read_stderr().to_string();
    if let Err(e) = res {
        stderr.push('\n');
        stderr.push_str(&e.to_string());
    }

    if bless_stream_files {
        write_file_if_nonempty(&base_path, "stdout", stdout);
        write_file_if_nonempty(&base_path, "stderr", &stderr);
    } else {
        let expected_stdout = read_file_or_empty(&base_path, "stdout");
        assert_eq!(stdout, expected_stdout);

        let expected_stderr = read_file_or_empty(&base_path, "stderr");
        assert_eq!(stderr, expected_stderr);
    }
}

pub const BLESS: Option<&'static str> = option_env!("BLESS");

make_tests! {}
