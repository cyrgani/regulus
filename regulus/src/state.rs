use crate::Directory;
use crate::builtins::all_functions;
use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Stderr, Stdout, Write, stderr, stdin, stdout};
use std::path::Path;
use std::{io, str};

#[derive(Clone)]
pub struct Storage {
    // TODO: consider a HashMap<String, (bool, Atom)> instead, the bool means local / global
    pub data: HashMap<String, Atom>,
    pub global_idents: HashSet<String>,
    pub exported_idents: HashSet<String>,
}

impl Storage {
    pub fn initial() -> Self {
        Self {
            data: all_functions(),
            global_idents: HashSet::new(),
            exported_idents: HashSet::new(),
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Atom> {
        self.data.get(name.as_ref())
    }

    pub fn insert(&mut self, name: impl AsRef<str>, value: Atom) {
        self.data.insert(name.as_ref().to_owned(), value);
    }

    pub fn global_items(&self) -> impl Iterator<Item = (String, Atom)> {
        self.data
            .iter()
            .filter(|(ident, _)| self.global_idents.contains(*ident))
            .map(|(ident, atom)| (ident.clone(), atom.clone()))
    }

    pub fn exported_and_global_items(&self) -> impl Iterator<Item = (String, Atom)> {
        self.data
            .iter()
            .filter(|(ident, _)| {
                self.global_idents.contains(*ident) || self.exported_idents.contains(*ident)
            })
            .map(|(ident, atom)| (ident.clone(), atom.clone()))
    }

    pub fn add_global(&mut self, name: impl AsRef<str>, value: Atom) {
        self.global_idents.insert(name.as_ref().to_owned());
        self.data.insert(name.as_ref().to_owned(), value);
    }
}

// TODO: users should be able to set their own stderr/out/in streams too
pub struct State {
    pub storage: Storage,
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
            storage: Storage::initial(),
            stdin: Box::new(BufReader::new(stdin())),
            stdout: WriteHandle::Regular(stdout()),
            stderr: WriteHandle::Regular(stderr()),
            file_directory: current_dir,
            exit_unwind_value: None,
        }
    }

    pub fn get_function(&self, name: &str) -> Result<Function> {
        match self.storage.data.get(name) {
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
            storage: Storage::initial(),
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
