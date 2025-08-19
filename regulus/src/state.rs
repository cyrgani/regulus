use crate::builtins::all_functions;
use crate::exception::NameError;
use crate::no_path;
use crate::parsing::positions::Span;
use crate::parsing::{build_program, tokenize};
use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write, stderr, stdin, stdout};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{env, fs, io, str};

#[derive(Clone)]
pub(crate) enum Directory {
    Regular(PathBuf),
    FromEval,
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
                    raise!(NameError, "`{name}` is not a function!")
                }
            }
            None => raise!(NameError, "No function `{name}` found!"),
        }
    }
}

// TODO: add and update all docs here!
pub struct State {
    pub storage: Storage,
    pub stdin: Box<dyn BufRead>,
    pub stdout: WriteHandle,
    pub stderr: WriteHandle,
    pub(crate) file_directory: Directory,
    pub(crate) current_file_path: Option<PathBuf>,
    pub(crate) exit_unwind_value: Option<Result<Atom>>,
    pub(crate) backtrace: Vec<Span>,
    code: Option<String>,
    next_type_id: i64,
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
            stdout: WriteHandle::new_write(stdout()),
            stderr: WriteHandle::new_write(stderr()),
            file_directory: Directory::InternedSTL,
            current_file_path: None,
            exit_unwind_value: None,
            backtrace: Vec::new(),
            code: None,
            next_type_id: Atom::MIN_OBJECT_TY_ID,
            __private: (),
        }
    }

    /// Sets the code that will be executed.
    #[must_use = "this returns the new state without modifying the original"]
    pub fn with_code(mut self, code: impl AsRef<str>) -> Self {
        self.code = Some(code.as_ref().to_owned());
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
        self.code = Some(fs::read_to_string(&path)?);

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

    /// Runs the given program with the details specified by this state.
    ///
    /// Returns the result the program returned.
    ///
    /// # Panics
    /// Panics if `code` was not set.
    pub fn run(&mut self) -> Result<Atom> {
        // newlines are needed to avoid interaction with comments
        // might also help with calculating the actual spans (just do line - 1)
        let prelude_import = match self.file_directory {
            Directory::Regular(_) | Directory::FromEval => "__builtin_prelude_import(),",
            Directory::InternedSTL => "",
        };

        let code = format!(
            "_({prelude_import}\n{}\n)",
            self.code
                .as_ref()
                .expect("setting the source code is required")
        );

        let file_path = if let Some(path) = &self.current_file_path {
            Rc::new(path.clone())
        } else {
            no_path()
        };

        let tokens = tokenize(&code, file_path)?;

        let program = build_program(tokens)?;

        let result = program.eval(self)?.into_owned();

        if let Some(exit_unwind_value) = &self.exit_unwind_value {
            return exit_unwind_value.clone();
        }

        Ok(result)
    }

    /// Writes the given string to stdout, without any extra newline.
    pub(crate) fn write_to_stdout(&mut self, msg: &str) {
        self.stdout.as_write().write_all(msg.as_bytes()).unwrap();
    }

    /// Writes the given string to stdout, without any extra newline.
    pub(crate) fn write_to_stderr(&mut self, msg: &str) {
        self.stderr.as_write().write_all(msg.as_bytes()).unwrap();
    }

    /// Only intended to be used by `import` internals for now.
    pub(crate) fn set_current_file_path(&mut self, path: impl AsRef<Path>) {
        self.current_file_path = Some(path.as_ref().to_owned());
    }

    /// Returns a new type id for a `type` call.
    pub const fn make_type_id(&mut self) -> i64 {
        let old = self.next_type_id;
        self.next_type_id += 1;
        old
    }

    pub(crate) fn raise(&self, error: impl Into<Error>, msg: impl Into<String>) -> Exception {
        Exception::with_trace(error, msg, &self.backtrace)
    }
}

/// Helper trait for types that can both be read from and written to.
pub trait ReadAndWrite: Read + Write {}

impl<T> ReadAndWrite for T where T: Read + Write {}

/// A handle that always allows writing and optionally allows reading. Used for stdout and stderr.
///
/// Usually, stdout / stderr only need to be written to, but sometimes, one may want to capture
/// what is written to them. In this case, the `ReadWrite` variant can be used.
pub enum WriteHandle {
    ReadWrite(Box<dyn ReadAndWrite>),
    Write(Box<dyn Write>),
}

impl WriteHandle {
    /// Access the `Write` part of this handle.
    pub fn as_write(&mut self) -> &mut dyn Write {
        match self {
            Self::Write(w) => w,
            Self::ReadWrite(w) => w,
        }
    }

    /// Constructs a new `WriteHandle` that only allows writing.
    pub fn new_write(write: impl Write + 'static) -> Self {
        Self::Write(Box::new(write))
    }

    /// Access the `Read` part of this handle or `None` if the handle does not allow reading.
    pub fn as_read(&mut self) -> Option<&mut dyn Read> {
        match self {
            Self::ReadWrite(w) => Some(w),
            Self::Write(_) => None,
        }
    }

    /// Constructs a new `WriteHandle` that allows both writing and reading.
    pub fn new_read_write(read_write: impl ReadAndWrite + 'static) -> Self {
        Self::ReadWrite(Box::new(read_write))
    }

    /// Return a string representation of the data in this handle if it allows reading.
    ///
    /// # Panics
    /// Panics if it is does not allow reading or if it does not contain valid UTF-8.
    pub fn read_to_string(&mut self) -> String {
        let Self::ReadWrite(buf) = self else {
            panic!("read_buffer() expected a buffer")
        };
        let mut vec = vec![0; 1024];
        let mut n = 0;
        let mut last_was_zero = false;
        loop {
            let amount = buf.read(&mut vec[n..]).unwrap();
            n += amount;
            if amount == 0 {
                if last_was_zero {
                    break;
                }
                vec.extend_from_slice(&[0; 1024]);
                last_was_zero = true;
            } else {
                last_was_zero = false;
            }
        }
        vec.retain(|&x| x != 0);
        String::from_utf8(vec).unwrap()
    }
}
