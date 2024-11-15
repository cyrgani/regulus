use crate::prelude::*;
use std::collections::HashMap;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Stderr, Stdout, Write};
use std::str;

pub struct State {
    pub storage: HashMap<String, Atom>,
    pub stdin: Box<dyn Send + Sync + BufRead>,
    pub stdout: WriteHandle<Stdout>,
    pub stderr: WriteHandle<Stderr>,
}

impl State {
    pub fn initial() -> Self {
        Self {
            storage: initial_storage(),
            stdin: Box::new(BufReader::new(stdin())),
            stdout: WriteHandle::Regular(stdout()),
            stderr: WriteHandle::Regular(stderr()),
        }
    }

    pub fn get_function(&self, name: &str) -> ProgResult<Function> {
        self.storage
            .values()
            .find_map(|atom| match atom {
                Atom::Function(function) if function.name == name => Some(function.clone()),
                _ => None,
            })
            .ok_or_else(|| Exception::new(format!("No function `{name}` found!"), Error::Name))
    }
}

pub fn initial_storage() -> HashMap<String, Atom> {
    crate::function::all_functions()
        .into_iter()
        .map(|f| (f.name.clone(), Atom::Function(f)))
        .collect()
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
    pub fn get_buffer(&self) -> &str {
        let Self::Buffer(buf) = self else {
            unreachable!()
        };
        str::from_utf8(buf).unwrap()
    }
}
