use crate::Directory;
use crate::builtins::all_functions;
use crate::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Stderr, Stdout, Write, stderr, stdin, stdout};
use std::path::Path;
use std::{io, str};

pub struct State {
    pub storage: HashMap<String, Atom>,
    stdin: Box<dyn BufRead>,
    stdout: WriteHandle<Stdout>,
    stderr: WriteHandle<Stderr>,
    pub(crate) file_directory: Directory,
    pub(crate) exit_unwind_value: Option<Result<Atom>>,
}

impl State {
    pub fn initial(current_dir: impl AsRef<Path>) -> Self {
        Self::initial_with_dir(Directory::Regular(current_dir.as_ref().to_path_buf()))
    }

    pub(crate) fn initial_with_dir(current_dir: Directory) -> Self {
        Self {
            storage: all_functions(),
            stdin: Box::new(BufReader::new(stdin())),
            stdout: WriteHandle::Regular(stdout()),
            stderr: WriteHandle::Regular(stderr()),
            file_directory: current_dir,
            exit_unwind_value: None,
        }
    }

    pub fn get_function(&self, name: &str) -> Result<Function> {
        match self.storage.get(name) {
            Some(atom) => {
                if let Atom::Function(func) = atom {
                    Ok(func.clone())
                } else {
                    raise!(Error::Name, "`{name}` is not a function!")
                }
            }
            None => raise!(Error::Name, "No function `{name}` found!"),
        }
    }

    pub(crate) fn stdin(&mut self) -> &mut dyn BufRead {
        &mut self.stdin
    }

    pub(crate) fn stdout(&mut self) -> &mut dyn Write {
        match &mut self.stdout {
            WriteHandle::Buffer(buf) => buf,
            WriteHandle::Regular(stdout) => stdout,
        }
    }

    #[expect(dead_code, reason = "nothing outputs to stderr yet")]
    pub(crate) fn stderr(&mut self) -> &mut dyn Write {
        match &mut self.stderr {
            WriteHandle::Buffer(buf) => buf,
            WriteHandle::Regular(stderr) => stderr,
        }
    }

    #[doc(hidden)]
    pub fn testing_setup(dir_path: &str, stdin: &str) -> Self {
        Self {
            storage: all_functions(),
            stdin: Box::new(BufReader::new(VecReader(stdin.as_bytes().to_vec()))),
            stdout: WriteHandle::Buffer(vec![]),
            stderr: WriteHandle::Buffer(vec![]),
            file_directory: Directory::Regular(dir_path.into()),
            exit_unwind_value: None,
        }
    }

    #[doc(hidden)]
    pub fn testing_read_stdout(&self) -> &str {
        self.stdout.read_buffer()
    }

    #[doc(hidden)]
    pub fn testing_read_stderr(&self) -> &str {
        self.stderr.read_buffer()
    }
}

struct VecReader(Vec<u8>);
impl Read for VecReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.as_slice().read(buf)
    }
}

enum WriteHandle<T> {
    Regular(T),
    Buffer(Vec<u8>),
}

impl<T> WriteHandle<T> {
    /// Return a string representation of this handle if it is a buffer.
    ///
    /// # Panics
    /// Panics if it is not a buffer or if it does not contain valid UTF-8.
    fn read_buffer(&self) -> &str {
        let Self::Buffer(buf) = self else {
            unreachable!()
        };
        str::from_utf8(buf).unwrap()
    }
}
