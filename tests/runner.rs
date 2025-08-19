use regulus::FILE_EXTENSION;
use regulus::prelude::{State, WriteHandle};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::str;
use std::{fs, io};
use tests::make_tests;

fn read_file_or_empty(base_path: &Path, extension: &str) -> String {
    fs::read_to_string(base_path.with_extension(extension)).unwrap_or_default()
}

fn delete_then_write_file_if_nonempty(base_path: &Path, extension: &str, content: &str) {
    let path = base_path.with_extension(extension);
    let _ = fs::remove_file(&path);
    if content.is_empty() {
        return;
    }
    fs::write(&path, content).unwrap();
}

/// Run a test program, making sure it produces the expected stdout and stderr.
pub fn run_test(dir_path: &str, name: &str) {
    //let mut overwrite_stream_files = env::args().any(|arg| arg == "--bless");
    let mut bless_stream_files = false;
    if let Some(var) = BLESS {
        if ["Y", "y", "yes", "true"].contains(&var) {
            bless_stream_files = true;
        }
    }

    let base_path = PathBuf::from(dir_path).join(name);
    let mut state = State::new()
        .with_source_file(base_path.with_extension(FILE_EXTENSION))
        .expect("fatal error: program file not found");
    state.stdin = Box::new(BufReader::new(RwVec(
        read_file_or_empty(&base_path, "stdin").into_bytes(),
    )));
    state.stdout = WriteHandle::new_read_write(RwVec(vec![]));
    state.stderr = WriteHandle::new_read_write(RwVec(vec![]));

    let res = state.run();

    let stdout = state.stdout.read_to_string().to_owned();
    let mut stderr = state.stderr.read_to_string().to_owned();

    if let Err(e) = res {
        stderr.push('\n');
        stderr.push_str(&e.to_string());
    }

    if bless_stream_files {
        delete_then_write_file_if_nonempty(&base_path, "stdout", &stdout);
        delete_then_write_file_if_nonempty(&base_path, "stderr", &stderr);
    } else {
        let expected_stderr = read_file_or_empty(&base_path, "stderr");
        assert_eq!(stderr, expected_stderr);

        let expected_stdout = read_file_or_empty(&base_path, "stdout");
        assert_eq!(stdout, expected_stdout);
    }
}

struct RwVec(Vec<u8>);

impl Write for RwVec {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for RwVec {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let amount = buf.write(&self.0)?;
        self.0 = self.0.split_off(amount);
        Ok(amount)
    }
}

pub const BLESS: Option<&'static str> = option_env!("BLESS");

make_tests! {}
