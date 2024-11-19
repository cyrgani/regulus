use tests_generator_macro::make_tests;

use newlang::prelude::{initial_storage, run, State, WriteHandle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use std::str;

#[derive(Deserialize, Serialize, Default)]
struct TestStreamData {
    stdin: String,
    stdout: String,
    stderr: String,
}

/// Run a test program, making sure it produces the expected stdout and stderr.
pub fn run_test(dir_path: &str, name: &str) {
    let mut overwrite_stream_files = false;
    if let Some(var) = OVERWRITE_STREAM_FILES {
        if ["y", "yes", "true"].contains(&var) {
            overwrite_stream_files = true;
        }
    }

    let source = fs::read_to_string(format!("{dir_path}/{name}.prog"))
        .expect("fatal error: program file not found");
    let data_path = format!("{dir_path}/{name}_streams.json");

    let data = match fs::read_to_string(&data_path) {
        Ok(streams_text) => serde_json::from_str::<TestStreamData>(&streams_text)
            .expect("fatal error: failed to parse stream file"),
        Err(err) => {
            if overwrite_stream_files {
                TestStreamData::default()
            } else {
                panic!("fatal error: stream file not found: {err}")
            }
        }
    };

    let (_, final_state) = run(
        &source,
        dir_path,
        Some(State {
            storage: initial_storage(),
            stdin: Box::new(BufReader::new(VecReader(data.stdin.as_bytes().to_vec()))),
            stdout: WriteHandle::Buffer(vec![]),
            stderr: WriteHandle::Buffer(vec![]),
            file_directory: PathBuf::from(dir_path),
        }),
    )
    .unwrap();

    let stdout = final_state.stdout.get_buffer();
    let stderr = final_state.stderr.get_buffer();
    // TODO: consider if this is a good idea
    assert!(stderr.is_empty());

    if overwrite_stream_files {
        let new_data = TestStreamData {
            stdin: String::new(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
        };
        fs::write(
            data_path,
            serde_json::to_vec(&new_data).expect("fatal error: failed to serialize stream data"),
        )
        .expect("fatal error: failed to overwrite stream file");
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

pub const OVERWRITE_STREAM_FILES: Option<&'static str> = option_env!("OVERWRITE_STREAM_FILES");

make_tests! {}
