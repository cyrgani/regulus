use crate::builtins::all_functions;
use crate::parsing::positions::{ExpandedSpan, Span};
use crate::parsing::{build_program, tokenize};
use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Stderr, Stdout, Write, stderr, stdin, stdout};
use std::path::{Path, PathBuf};
use std::{env, fs, io, str};

// TODO: maybe replace this with Option<PathBuf>
#[derive(Clone)]
pub(crate) enum Directory {
    Regular(PathBuf),
    /// Should only be used internally.
    InternedSTL,
}

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
    current_file_path: Option<PathBuf>,
    pub(crate) exit_unwind_value: Option<Result<Atom>>,
    /// TODO: not updated yet
    #[expect(dead_code, reason = "WIP")]
    current_span: Span,
    code: String,
    code_was_initialized: bool,
    file_path_indices: Vec<PathBuf>,
    // make sure this type can never be constructed from outside
    __private: (),
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// Creates a new state for running a program.
    ///
    /// You must use a method such as [`with_code`](Self::with_code) or [`with_source_file`](Self::with_source_file)
    /// to set the program source code before using [`run`](Self::run) to execute it.
    pub fn new() -> Self {
        Self {
            storage: Storage::initial(),
            stdin: Box::new(BufReader::new(stdin())),
            stdout: WriteHandle::Regular(stdout()),
            stderr: WriteHandle::Regular(stderr()),
            file_directory: Directory::InternedSTL,
            current_file_path: None,
            exit_unwind_value: None,
            current_span: Span::new(0, 0, 0),
            code: String::new(),
            code_was_initialized: false,
            file_path_indices: vec![],
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

        self.current_file_path = Some(path.as_ref().to_owned());

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

    /// Asserts that the source code is already initialized.
    fn assert_code_init(&self) {
        assert!(
            self.code_was_initialized,
            "setting the source code is required"
        );
    }

    /// Runs the given program with the details specified by this state.
    ///
    /// Returns the result the program returned.
    ///
    /// # Panics
    /// Panics if `code` was not set.
    pub fn run(&mut self) -> Result<Atom> {
        self.assert_code_init();

        // newlines are needed to avoid interaction with comments
        // might also help with calculating the actual spans (just do line - 1)
        self.code = format!("_(\n{}\n)", self.code);

        let file_id = if let Some(path) = &self.current_file_path {
            self.add_file_to_index(path.clone())
        } else {
            u16::MAX
        };

        let tokens = tokenize(&self.code, file_id)?;

        let program = build_program(tokens)?;

        let result = program.eval(self)?.into_owned();

        if let Some(exit_unwind_value) = &self.exit_unwind_value {
            return exit_unwind_value.clone();
        }

        Ok(result)
    }

    // TODO: consider removing the three methods below and making the corresponding three fields public instead

    /// Returns a mutable reference to the currently set stdin, allowing you to replace or update it.
    pub const fn stdin(&mut self) -> &mut Box<dyn BufRead> {
        &mut self.stdin
    }

    /// Returns a mutable reference to the currently set stdout, allowing you to replace or update it.
    pub const fn stdout(&mut self) -> &mut WriteHandle<Stdout> {
        &mut self.stdout
    }

    /// Returns a mutable reference to the currently set stderr, allowing you to replace or update it.
    pub const fn stderr(&mut self) -> &mut WriteHandle<Stderr> {
        &mut self.stderr
    }

    /// Writes the given string to stdout, without any extra newline.
    pub(crate) fn write_to_stdout(&mut self, msg: &str) {
        self.stdout.write_all(msg.as_bytes()).unwrap();
    }

    /// Returns an immutable reference to the source code.
    ///
    /// # Panics
    /// Panics if the source code was not yet initialized.
    pub(crate) fn code(&self) -> &str {
        self.assert_code_init();
        &self.code
    }

    /// Adds the given file path to the index of file paths.
    /// Returns the index this path has now.
    pub(crate) fn add_file_to_index(&mut self, path: impl AsRef<Path>) -> u16 {
        self.file_path_indices.push(path.as_ref().to_path_buf());
        u16::try_from(self.file_path_indices.len()).unwrap() - 1
    }

    /// Resolves a file path index to a [`PathBuf`].
    ///
    /// TODO: as a temporary measure, this returns an empty [`PathBuf`] when [`u16::MAX`] is passed.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub(crate) fn resolve_file_index(&self, index: u16) -> PathBuf {
        if index == u16::MAX {
            return PathBuf::new();
        }
        self.file_path_indices[index as usize].clone()
    }

    // TODO: choose which of the two methods below is more useful

    /// Returns the span of the source code part which is currently being interpreted.
    ///
    /// Useful for error messages.
    pub(crate) const fn current_span(&self) -> Span {
        self.current_span
    }

    /// Returns the expanded of the source code part which is currently being interpreted.
    ///
    /// Useful for error messages.
    pub(crate) fn current_span_expanded(&self) -> ExpandedSpan {
        self.current_span.expand(self)
    }

    /// Only intended to be used by `import` internals for now.
    pub(crate) fn set_current_file_path(&mut self, path: impl AsRef<Path>) {
        self.current_file_path = Some(path.as_ref().to_owned());
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
            panic!("read_buffer() expected a buffer")
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
