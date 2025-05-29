use crate::Directory;
use crate::build_program;
use crate::builtins::all_functions;
use crate::parsing::positions::Position;
use crate::prelude::*;
use crate::tokenize;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Stderr, Stdout, Write, stderr, stdin, stdout};
use std::path::{Path, PathBuf};
use std::{env, fs, io, str};

pub struct Storage {
    // TODO: consider a HashMap<String, (bool, Atom)> instead, the bool means local / global
    pub data: HashMap<String, Atom>,
    pub global_idents: HashSet<String>,
}

impl Storage {
    pub fn initial() -> Self {
        Self {
            data: all_functions(),
            global_idents: HashSet::new(),
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Atom> {
        self.data.get(name.as_ref())
    }

    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<Atom> {
        self.data.remove(name.as_ref())
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

    pub fn add_global(&mut self, name: impl AsRef<str>, value: Atom) {
        self.global_idents.insert(name.as_ref().to_owned());
        self.data.insert(name.as_ref().to_owned(), value);
    }

    // TODO: this function is weird
    pub(crate) fn get_function(&self, name: &str) -> Result<Function> {
        match self.get(name) {
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
}

// TODO: add and update all docs here!
pub struct State {
    pub storage: Storage,
    stdin: Box<dyn BufRead>,
    stdout: WriteHandle<Stdout>,
    stderr: WriteHandle<Stderr>,
    pub(crate) file_directory: Directory,
    pub(crate) exit_unwind_value: Option<Result<Atom>>,
    /// TODO: not updated yet
    #[expect(dead_code, reason = "WIP")]
    pub(crate) current_pos: Position,
    pub(crate) code: String,
    code_was_initialized: bool,
    // make sure this type can never be constructed from outside
    __private: (),
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            storage: Storage::initial(),
            stdin: Box::new(BufReader::new(stdin())),
            stdout: WriteHandle::Regular(stdout()),
            stderr: WriteHandle::Regular(stderr()),
            file_directory: Directory::InternedSTL,
            exit_unwind_value: None,
            current_pos: Position::ONE,
            code: String::new(),
            code_was_initialized: false,
            __private: (),
        }
    }

    /// Sets the code that will be executed.
    #[must_use = "this returns the new state without modifying the original"]
    pub fn with_code(mut self, code: impl AsRef<str>) -> Self {
        code.as_ref().clone_into(&mut self.code);
        self.code_was_initialized = true;
        self
    }

    /// Sets both the code and the current directory by reading from the given file path.
    ///
    /// # Errors
    /// Returns an error if reading from the file failed.
    ///
    /// # Panics
    /// Panics if the path is invalid.
    #[must_use = "this returns the new state without modifying the original"]
    pub fn with_source_file(mut self, path: impl AsRef<Path>) -> io::Result<Self> {
        self.code = fs::read_to_string(&path)?;
        self.code_was_initialized = true;

        let mut current_dir = path.as_ref().parent().unwrap().to_path_buf();
        if current_dir == PathBuf::new() {
            current_dir = PathBuf::from(".");
        }
        self.file_directory = Directory::Regular(current_dir);
        Ok(self)
    }

    /// Sets the source directory for resolving imports to the given directory.
    #[must_use = "this returns the new state without modifying the original"]
    pub fn with_source_directory(mut self, dir_path: impl AsRef<Path>) -> Self {
        self.file_directory = Directory::Regular(dir_path.as_ref().to_path_buf());
        self
    }

    /// Sets the current directory to the operating systems current working directory.
    ///
    /// # Panics
    /// Panics if [`env::current_dir`] returned an error.
    #[must_use = "this returns the new state without modifying the original"]
    pub fn with_cwd(self) -> Self {
        self.with_source_directory(env::current_dir().unwrap())
    }

    /// Run the program specified by this configuration.
    ///
    /// Returns the result the program returned and the final state.
    ///
    /// If `starting_state` is specified, it overrides `current_dir`.
    ///
    /// # Panics
    /// Panics if the configuration is invalid.
    /// This happens if one of the following cases occurs:
    /// * `code` is missing
    pub fn run(mut self) -> (Result<Atom>, Self) {
        assert!(
            self.code_was_initialized,
            "setting the source code is required"
        );

        macro_rules! return_err {
            ($val: expr) => {
                match $val {
                    Ok(ok) => ok,
                    Err(err) => return (Err(err), self),
                }
            };
        }

        // newlines are needed to avoid interaction with comments
        // might also help with calculating the actual spans (just do line - 1)
        self.code = format!("_(\n{}\n)", self.code);

        let tokens = return_err!(tokenize(&self.code));

        let program = return_err!(build_program(tokens));

        let result = return_err!(program.eval(&mut self)).into_owned();

        if let Some(exit_unwind_value) = &self.exit_unwind_value {
            return (exit_unwind_value.clone(), self);
        }

        (Ok(result), self)
    }

    pub fn stdin(&mut self) -> &mut Box<dyn BufRead> {
        &mut self.stdin
    }

    pub const fn stdout(&mut self) -> &mut WriteHandle<Stdout> {
        &mut self.stdout
    }

    pub const fn stderr(&mut self) -> &mut WriteHandle<Stderr> {
        &mut self.stderr
    }
}

// TODO: think about whether there is a design that allows avoiding this type
/// A handle that is either a `Vec<u8>` (to allow reading from it later)
/// or anything that implements `Write` (such as `Stdout` or `Stderr`).
pub enum WriteHandle<T: Write> {
    Regular(T),
    Buffer(Vec<u8>),
}

impl<T: Write> WriteHandle<T> {
    /// Return a string representation of this handle if it is a buffer.
    ///
    /// # Panics
    /// Panics if it is not a buffer or if it does not contain valid UTF-8.
    pub fn read_buffer(&self) -> &str {
        let Self::Buffer(buf) = self else {
            unreachable!()
        };
        str::from_utf8(buf).unwrap()
    }
}

impl<T: Write> Write for WriteHandle<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Regular(t) => t.write(buf),
            Self::Buffer(vec) => vec.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Regular(t) => t.flush(),
            Self::Buffer(vec) => vec.flush(),
        }
    }
}
