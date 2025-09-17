use crate::builtins::all_functions;
use crate::exception::NameError;
use crate::no_path;
use crate::optimizations::optimize;
use crate::parsing::Span;
use crate::parsing::{build_program, tokenize};
use crate::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
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

pub enum StoredValue {
    Global(Atom),
    /// An identifier may refer to any number of atoms within different scopes.
    /// Only the innermost one will be considered, until its scope ends.
    Locals(Vec<(usize, Atom)>),
}

impl StoredValue {
    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Self::Global(a) => Some(a),
            Self::Locals(v) => v.last().map(|(_, a)| a),
        }
    }

    pub fn update(&mut self, atom: Atom, scope: usize) {
        match self {
            Self::Global(a) => *a = atom,
            Self::Locals(vec) => {
                if let Some(last) = vec.last()
                    && last.0 == scope
                {
                    vec.pop();
                }
                vec.push((scope, atom));
            }
        }
    }

    pub fn reduce_by_scope(&mut self, scope: usize) {
        if let Self::Locals(vec) = self
            && let Some(last) = vec.last()
            && last.0 == scope
        {
            let _ = vec.pop();
        }
    }
}

pub struct Storage {
    pub data: HashMap<String, StoredValue>,
    pub current_scope: usize,
}

impl Storage {
    pub fn initial() -> Self {
        Self {
            data: all_functions()
                .into_iter()
                .map(|(name, f)| (name, StoredValue::Locals(vec![(0, f)])))
                .collect(),
            current_scope: 0,
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Atom> {
        self.data.get(name.as_ref())?.as_atom()
    }

    pub fn insert(&mut self, name: impl AsRef<str>, value: Atom) {
        match self.data.entry(name.as_ref().to_string()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().update(value, self.current_scope);
            }
            Entry::Vacant(entry) => {
                entry.insert(StoredValue::Locals(vec![(self.current_scope, value)]));
            }
        }
    }

    pub fn add_global(&mut self, name: impl AsRef<str>, value: Atom) {
        self.data
            .insert(name.as_ref().to_string(), StoredValue::Global(value));
    }

    #[expect(
        clippy::missing_const_for_fn,
        reason = "type is not constructible in const anyway"
    )]
    pub fn start_scope(&mut self) {
        self.current_scope += 1;
    }

    pub fn end_scope(&mut self) {
        for val in self.data.values_mut() {
            val.reduce_by_scope(self.current_scope);
        }
        self.current_scope -= 1;
    }

    pub fn extend_from(&mut self, other: Self) {
        assert_eq!(other.current_scope, 0);
        for (name, value) in other.data {
            match value {
                StoredValue::Global(global) => {
                    self.add_global(name, global);
                }
                StoredValue::Locals(mut locals) => {
                    assert_eq!(locals.len(), 1);
                    self.insert(name, locals.pop().unwrap().1);
                }
            }
        }
    }
}

// TODO: add and update all docs here as well as on `Storage`.
/// The central structure for running a program.
pub struct State {
    /// All values that can be accessed during the program's execution.
    pub storage: Storage,
    /// Handle to the standard input. Defaults to [`std::io::stdin()`], but can be replaced.
    pub stdin: Box<dyn BufRead>,
    /// Handle to the standard output. Defaults to [`std::io::stdout()`], but can be replaced.
    pub stdout: WriteHandle,
    /// Handle to the standard error. Defaults to [`std::io::stderr()`], but can be replaced.
    pub stderr: WriteHandle,
    /// The directory (or pseudo-directory) in which the current program is placed.
    pub(crate) file_directory: Directory,
    pub(crate) current_file_path: Option<PathBuf>,
    pub(crate) exit_unwind_value: Option<Result<Atom>>,
    pub(crate) backtrace: Vec<Span>,
    // TODO: consider merging `current_doc_comment` and `current_fn_name`
    pub(crate) current_doc_comment: Option<String>,
    pub(crate) current_fn_name: Option<String>,
    /// Tracks the current stack of nested `import`-calls to emit an error on cyclic imports.
    /// Note that this only operates on user-written code and does not catch cyclic import
    /// errors within the STL (those still cause a rust stack overflow).
    pub(crate) import_stack: Vec<PathBuf>,
    code: Option<String>,
    next_type_id: i64,
    optimizations_enabled: bool,
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
            current_doc_comment: None,
            current_fn_name: None,
            import_stack: Vec::new(),
            code: None,
            next_type_id: Atom::MIN_OBJECT_TY_ID,
            optimizations_enabled: false,
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
    #[must_use = "this returns the new state without modifying the original"]
    pub fn with_source_file(mut self, path: impl AsRef<Path>) -> io::Result<Self> {
        self.code = Some(fs::read_to_string(&path)?);

        self.current_file_path = Some(path.as_ref().to_owned());

        let mut current_dir = path
            .as_ref()
            .parent()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidFilename, "file path has no parent")
            })?
            .to_path_buf();

        if current_dir == PathBuf::new() {
            current_dir = PathBuf::from(".");
        }
        self.file_directory = Directory::Regular(current_dir);
        Ok(self)
    }

    /// Sets the source directory for resolving imports to the given directory.
    ///
    /// Note that this does not set or change the program code.
    #[must_use = "this returns the new state without modifying the original"]
    pub fn with_source_directory(mut self, dir_path: impl AsRef<Path>) -> Self {
        self.file_directory = Directory::Regular(dir_path.as_ref().to_path_buf());
        self
    }

    /// Enables optimizations which run prior to execution.
    ///
    /// **WARNING**: Enabling optimizations may cause programs that redefine builtins
    /// (or possibly also STL functions) to change their behavior.
    #[expect(
        clippy::missing_const_for_fn,
        reason = "type cannot be constructed in const anyway"
    )]
    #[must_use = "this returns the new state without modifying the original"]
    pub fn enable_optimizations(mut self) -> Self {
        self.optimizations_enabled = true;
        self
    }

    /// Sets the current directory to the operating systems current working directory.
    ///
    /// Note that this does not set or change the program code.
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
        // and also help with calculating the actual spans (just do line - 1)
        let code = format!(
            "_(__builtin_prelude_import(),\n{}\n)",
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

        let mut program = build_program(tokens)?;
        if self.optimizations_enabled {
            optimize(&mut program);
        }

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

    /// Constructs a new exception with the given error and message at the current point of execution.
    pub fn raise(&self, error: impl Into<String>, msg: impl Into<String>) -> Exception {
        Exception::with_trace(error, msg, &self.backtrace)
    }

    // TODO: this function is weird
    pub(crate) fn get_function(&self, name: &str) -> Result<Function> {
        match self.storage.get(name) {
            Some(atom) => {
                if let Atom::Function(func) = atom {
                    Ok(func.clone())
                } else {
                    raise!(self, NameError, "`{name}` is not a function!")
                }
            }
            None => raise!(self, NameError, "No function `{name}` found!"),
        }
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
            panic!("read_to_string(): cannot read from write only handle")
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
