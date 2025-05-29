use regulus::prelude::{State, WriteHandle};
use regulus::{FILE_EXTENSION};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::str;
use std::{fs, io};
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
    let mut starting_state = State::new()
        .with_source_file(base_path.with_extension(FILE_EXTENSION))
        .expect("fatal error: program file not found");
    *starting_state.stdin() = Box::new(BufReader::new(VecReader(
        read_file_or_empty(&base_path, "stdin").into_bytes(),
    )));
    *starting_state.stdout() = WriteHandle::Buffer(vec![]);
    *starting_state.stderr() = WriteHandle::Buffer(vec![]);

    let (res, mut final_state) = starting_state.run();

    let stdout = final_state.stdout().read_buffer().to_owned();
    let mut stderr = final_state.stderr().read_buffer().to_owned();
    
    if let Err(e) = res {
        stderr.push('\n');
        stderr.push_str(&e.display(&final_state).to_string());
    }

    if bless_stream_files {
        write_file_if_nonempty(&base_path, "stdout", &stdout);
        write_file_if_nonempty(&base_path, "stderr", &stderr);
    } else {
        let expected_stdout = read_file_or_empty(&base_path, "stdout");
        assert_eq!(stdout, expected_stdout);

        let expected_stderr = read_file_or_empty(&base_path, "stderr");
        assert_eq!(stderr, expected_stderr);
    }
}

struct VecReader(Vec<u8>);
impl Read for VecReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.as_slice().read(buf)
    }
}

pub const BLESS: Option<&'static str> = option_env!("BLESS");

make_tests! {}
