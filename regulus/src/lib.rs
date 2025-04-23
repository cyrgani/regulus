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
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

mod argument;
mod atom;
mod exception;
mod function;
mod macros;
mod parsing;
mod state;

mod builtins;

pub mod prelude {
    pub use crate::{
        Runner,
        argument::{Argument, ArgumentData},
        atom::Atom,
        exception::{Error, Exception, Result},
        function::{Function, FunctionCall},
        functions, raise, run,
        state::State,
    };
}

use crate::{
    atom::Atom,
    exception::Result,
    parsing::{build_program, tokenize, validate_tokens},
    state::State,
};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub const FILE_EXTENSION: &str = "re";

/// A set of options required for running a Regulus program.
///
/// Only `code` must be specified,
/// `current_dir` and `starting_state` have default values.
#[must_use]
pub struct Runner {
    code: Option<String>,
    current_dir: Directory,
    starting_state: Option<State>,
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

impl Runner {
    pub const fn new() -> Self {
        Self {
            code: None,
            current_dir: Directory::InternedSTL,
            starting_state: None,
        }
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
        self.current_dir = Directory::Regular(current_dir);
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
        self.current_dir = Directory::Regular(dir_path.as_ref().to_path_buf());
        self
    }

    pub fn starting_state(mut self, state: State) -> Self {
        self.starting_state = Some(state);
        self
    }

    /// Run the program specified by this configuration.
    ///
    /// Returns the result the program returned and the final state.
    ///
    /// If `starting_state` is specified, it overrides `current_dir`..
    ///
    /// # Panics
    /// Panics if the configuration is invalid.
    /// This happens if one of the following cases occurs:
    /// * `code` is missing
    pub fn run(self) -> (Result<Atom>, State) {
        let code = self.code.expect("code is required");
        let mut state = self
            .starting_state
            .unwrap_or_else(|| State::initial_with_dir(self.current_dir));

        macro_rules! return_err {
            ($val: expr) => {
                match $val {
                    Ok(ok) => ok,
                    Err(err) => return (Err(err), state),
                }
            };
        }

        let tokens = return_err!(tokenize(&code));

        return_err!(validate_tokens(&tokens));

        let program = return_err!(build_program(&tokens, "_"));

        let result = return_err!(program.eval(&mut state));

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
/// For more options, use [`Runner`] instead.
///
/// # Panics
/// Panics if the path is invalid or cannot be read from.
pub fn run(path: impl AsRef<Path>) -> Result<Atom> {
    Runner::new().file(path).unwrap().run().0
}

mod interned_stdlib;
