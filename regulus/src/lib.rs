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

pub(crate) const STL_DIR: &str = "regulus/stdlib";

/// A set of options required for running a Regulus program.
///
/// Only `code` and `current_dir` must be specified,
/// `stl_dir` and `starting_state` have default values.
#[derive(Default)]
#[must_use]
pub struct Runner {
    code: Option<String>,
    current_dir: Option<PathBuf>,
    stl_dir: Option<PathBuf>,
    starting_state: Option<State>,
}

impl Runner {
    pub fn new() -> Self {
        Self::default()
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
        self.current_dir = Some(current_dir);
        Ok(self)
    }

    pub fn code(mut self, code: impl AsRef<str>) -> Self {
        self.code = Some(code.as_ref().to_owned());
        self
    }

    pub fn current_dir(mut self, dir_path: impl AsRef<Path>) -> Self {
        self.current_dir = Some(dir_path.as_ref().to_path_buf());
        self
    }

    pub fn stl_dir(mut self, stl_dir: impl AsRef<Path>) -> Self {
        self.stl_dir = Some(stl_dir.as_ref().to_path_buf());
        self
    }

    pub fn starting_state(mut self, state: State) -> Self {
        self.starting_state = Some(state);
        self
    }

    /// Run the program specified by this configuration.
    ///
    /// Returns `None` if the configuration is invalid (missing `code` or `current_dir`).
    ///
    /// Otherwise, returns the result the program returned and the final state.
    pub fn run(self) -> Option<(Result<Atom>, State)> {
        let code = self.code?;
        let current_dir = self.current_dir?;
        let stl_dir = self.stl_dir.unwrap_or_else(|| PathBuf::from(STL_DIR));
        Some(run_with_options(
            &code,
            current_dir,
            self.starting_state,
            stl_dir,
        ))
    }
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
    Runner::new()
        .file(path)
        .unwrap()
        .run()
        .expect("unreachable")
        .0
}

pub fn run_with_options(
    code: &str,
    dir: impl AsRef<Path>,
    start_state: Option<State>,
    stl_dir: impl AsRef<Path>,
) -> (Result<Atom>, State) {
    let mut state = start_state.unwrap_or_else(|| State::initial(dir, stl_dir));

    macro_rules! return_err {
        ($val: expr) => {
            match $val {
                Ok(ok) => ok,
                Err(err) => return (Err(err), state),
            }
        };
    }

    let tokens = return_err!(tokenize(code));

    return_err!(validate_tokens(&tokens));

    let program = return_err!(build_program(&tokens, "_"));

    let result = return_err!(program.eval(&mut state));
    (Ok(result), state)
}
