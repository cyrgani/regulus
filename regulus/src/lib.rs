#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::dbg_macro,
    clippy::allow_attributes
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::option_if_let_else,
    clippy::must_use_candidate
)]

mod argument;
mod atom;
mod exception;
mod function;
mod macros;
mod parsing;
mod state;

mod builtins;

#[rustfmt::skip]
mod interned_stdlib;

// TODO: reconsider and redesign the prelude, differentiate between internal and external usage
pub mod prelude {
    pub use crate::{
        argument::{Argument, ArgumentData},
        atom::Atom,
        exception::{Error, Exception, Result},
        function::{Function, FunctionBody, FunctionCall},
        functions, raise, run, run_file,
        state::{State, WriteHandle},
    };
}

use crate::state::Storage;
use crate::{
    atom::Atom,
    exception::Result,
    parsing::{build_program, tokenize},
    state::State,
};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub const FILE_EXTENSION: &str = "re";

// TODO: current idea: get rid of `Runner`, move all its relevant methods to `State`,
//  then there will be `State::run` and so on

/// TODO: update all docs for `Runner`.
/// A set of options required for running a Regulus program.
///
/// Only `code` must be specified,
/// `current_dir` and `starting_state` have default values.
#[must_use]
#[deprecated]
pub struct Runner {
    code: Option<String>,
    starting_state: State,
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

impl Runner {
    pub fn new() -> Self {
        panic!()
        /*Self {
            code: None,
            starting_state: State::initial_with_dir(Directory::InternedSTL),
        }*/
    }

    /// Sets both the code and the current directory by reading from the given file path.
    ///
    /// # Errors
    /// Returns an error if reading from the file failed.
    ///
    /// # Panics
    /// Panics if the path is invalid.
    pub fn file(mut self, path: impl AsRef<Path>) -> io::Result<Self> {
        self.code = Some(fs::read_to_string(&path)?);
        let mut current_dir = path.as_ref().parent().unwrap().to_path_buf();
        if current_dir == PathBuf::new() {
            current_dir = PathBuf::from(".");
        }
        self.starting_state.file_directory = Directory::Regular(current_dir);
        Ok(self)
    }

    pub fn code(mut self, code: impl AsRef<str>) -> Self {
        self.code = Some(code.as_ref().to_owned());
        self
    }

    /// Sets the current directory to the given directory path.
    ///
    /// This is used to resolve imports of other local files in the same directory.
    pub fn current_dir(mut self, dir_path: impl AsRef<Path>) -> Self {
        self.starting_state.file_directory = Directory::Regular(dir_path.as_ref().to_path_buf());
        self
    }

    /// Sets the current directory to the operating systems current working directory.
    ///
    /// # Panics
    /// Panics if [`env::current_dir`] returned an error.
    pub fn with_cwd(self) -> Self {
        self.current_dir(env::current_dir().unwrap())
    }

    #[deprecated]
    pub fn starting_state(mut self, state: State) -> Self {
        self.starting_state = state;
        self
    }

    pub fn starting_storage(mut self, storage: Storage) -> Self {
        self.starting_state.storage = storage;
        self
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
    pub fn run(self) -> (Result<Atom>, State) {
        let code = self.code.expect("code is required");
        let mut state = self.starting_state;

        macro_rules! return_err {
            ($val: expr) => {
                match $val {
                    Ok(ok) => ok,
                    Err(err) => return (Err(err), state),
                }
            };
        }

        // newlines are needed to avoid interaction with comments
        // might also help with calculating the actual spans (just do line - 1)
        let wrapped_code = format!("_(\n{code}\n)");
        state.code = wrapped_code;

        let tokens = return_err!(tokenize(&state.code));

        let program = return_err!(build_program(tokens));

        let result = return_err!(program.eval(&mut state)).into_owned();

        if let Some(exit_unwind_value) = &state.exit_unwind_value {
            return (exit_unwind_value.clone(), state);
        }

        (Ok(result), state)
    }
}

#[derive(Clone)]
pub(crate) enum Directory {
    Regular(PathBuf),
    /// Should only be used internally.
    InternedSTL,
}

/// A convenient helper for directly running one file program.
///
/// Returns only the result of running the program, not the final state.
///
/// For more options, use [`State`] instead.
///
/// # Panics
/// Panics if the path is invalid or cannot be read from.
pub fn run_file(path: impl AsRef<Path>) -> Result<Atom> {
    State::new().with_source_file(path).unwrap().run().0
}

/// A convenient helper for directly running a program string.
///
/// Returns only the result of running the program, not the final state.
///
/// For more options, use [`State`] instead.
pub fn run(code: impl AsRef<str>) -> Result<Atom> {
    State::new().with_code(code).run().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_value_program_return() {
        assert_eq!(
            State::new()
                .with_code("_(4)")
                .run()
                .0
                .unwrap()
                .int()
                .unwrap(),
            4
        );
        assert_eq!(
            State::new().with_code("4").run().0.unwrap().int().unwrap(),
            4
        );
        assert_eq!(
            State::new()
                .with_code("=(x, 4), x")
                .run()
                .0
                .unwrap()
                .int()
                .unwrap(),
            4
        );
    }
}
/*
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

pub static STRING_CLONE_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static LIST_CLONE_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static FUNCTION_CLONE_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static OBJECT_CLONE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn clone_investigate(atom: &Atom) {
    match atom {
        Atom::String(_) => {
            STRING_CLONE_COUNT.fetch_add(1, Relaxed);
        }
        Atom::List(_) => {
            LIST_CLONE_COUNT.fetch_add(1, Relaxed);
        }
        Atom::Function(_) => {
            FUNCTION_CLONE_COUNT.fetch_add(1, Relaxed);
        }
        Atom::Object(_) => {
            OBJECT_CLONE_COUNT.fetch_add(1, Relaxed);
        }
        Atom::Null | Atom::Int(_) | Atom::Bool(_) => (),
    }
}
*/
