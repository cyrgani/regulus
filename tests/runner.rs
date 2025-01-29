use tests_generator_macro::make_tests;

use newlang::prelude::{initial_storage, run, State, WriteHandle};
use std::fs;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::str;

#[derive(Default)]
struct TestStreamData {
    stdin: String,
    stdout: String,
    stderr: String,
}

fn read_file_or_empty(base_path: impl AsRef<Path>, extension: &str) -> String {
    fs::read_to_string(PathBuf::new().join(base_path).with_extension(extension)).unwrap_or_default()
}

fn write_file_if_nonempty(base_path: impl AsRef<Path>, extension: &str, content: &str) {
    if content.is_empty() {
        return;
    }
    fs::write(
        PathBuf::new().join(base_path).with_extension(extension),
        content,
    )
    .unwrap();
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
    
    let source = fs::read_to_string(base_path.with_extension("prog"))
        .expect("fatal error: program file not found");

    let data = TestStreamData {
        stdin: read_file_or_empty(&base_path, "stdin"),
        stdout: read_file_or_empty(&base_path, "stdout"),
        stderr: read_file_or_empty(&base_path, "stderr"),
    };

    let (res, final_state) = run(
        &source,
        dir_path,
        Some(State {
            storage: initial_storage(),
            stdin: Box::new(BufReader::new(VecReader(data.stdin.as_bytes().to_vec()))),
            stdout: WriteHandle::Buffer(vec![]),
            stderr: WriteHandle::Buffer(vec![]),
            file_directory: PathBuf::from(dir_path),
        }),
    );

    let stdout = final_state.stdout.read_buffer();
    let mut stderr = final_state.stderr.read_buffer().to_string();
    if let Err(e) = res {
        stderr.push('\n');
        stderr.push_str(&e.to_string());
    }

    if bless_stream_files {
        write_file_if_nonempty(&base_path, "stdout", stdout);
        write_file_if_nonempty(&base_path, "stderr", &stderr);
    } else {
        assert_eq!(stdout, data.stdout);
        assert_eq!(stderr, data.stderr);
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
