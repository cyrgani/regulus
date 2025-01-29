use crate::builtins::all_functions;
use crate::prelude::*;
use std::collections::HashMap;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Stderr, Stdout, Write};
use std::path::{Path, PathBuf};
use std::str;

pub struct State {
    pub storage: HashMap<String, Atom>,
    pub stdin: Box<dyn Send + Sync + BufRead>,
    pub stdout: WriteHandle<Stdout>,
    pub stderr: WriteHandle<Stderr>,
    pub file_directory: PathBuf,
}

impl State {
    pub fn initial(dir: impl AsRef<Path>) -> Self {
        Self {
            storage: initial_storage(),
            stdin: Box::new(BufReader::new(stdin())),
            stdout: WriteHandle::Regular(stdout()),
            stderr: WriteHandle::Regular(stderr()),
            file_directory: PathBuf::from(dir.as_ref()),
        }
    }

    pub fn get_function(&self, name: &str) -> Result<Function> {
        match self.storage.get(name) {
            Some(atom) => {
                if let Atom::Function(func) = atom {
                    Ok(func.clone())
                } else {
                    Exception::new_err(format!("`{name}` is not a function!"), Error::Name)
                }
            }
            None => Exception::new_err(format!("No function `{name}` found!"), Error::Name),
        }
    }
}

/// Constructs the initial storage at startup.
pub fn initial_storage() -> HashMap<String, Atom> {
    all_functions()
}

pub enum WriteHandle<T> {
    Regular(T),
    Buffer(Vec<u8>),
}

impl<T: Write> WriteHandle<T> {
    pub(crate) fn write_all(&mut self, buf: &[u8]) {
        match self {
            Self::Regular(val) => val.write_all(buf).unwrap(),
            Self::Buffer(val) => val.write_all(buf).unwrap(),
        }
    }

    /// Return a string representation of this handle if it is a buffer.
    ///
    /// # Panics
    /// Panics if it is not a buffer.
    pub fn read_buffer(&self) -> &str {
        let Self::Buffer(buf) = self else {
            unreachable!()
        };
        str::from_utf8(buf).unwrap()
    }
}
