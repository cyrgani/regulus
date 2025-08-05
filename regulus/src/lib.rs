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
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::needless_pass_by_value
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
        FILE_EXTENSION,
        argument::Argument,
        atom::{Atom, Object},
        exception::{Error, Exception, Result},
        function::{Function, FunctionBody, FunctionCall},
        functions, raise, run, run_file,
        state::{State, WriteHandle},
    };
}

pub(crate) fn no_path() -> Rc<PathBuf> {
    Rc::new(PathBuf::new())
}

use crate::{atom::Atom, exception::Result, state::State};
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub const FILE_EXTENSION: &str = "re";

/// A convenient helper for directly running one file program.
///
/// Returns only the result of running the program, not the final state.
///
/// For more options, use [`State`] instead.
///
/// # Panics
/// Panics if the path is invalid or cannot be read from.
pub fn run_file(path: impl AsRef<Path>) -> Result<Atom> {
    State::new().with_source_file(path).unwrap().run()
}

/// A convenient helper for directly running a program string.
///
/// Returns only the result of running the program, not the final state.
///
/// For more options, use [`State`] instead.
pub fn run(code: impl AsRef<str>) -> Result<Atom> {
    State::new().with_code(code).run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_value_program_return() {
        assert_eq!(
            State::new().with_code("_(4)").run().unwrap().int().unwrap(),
            4
        );
        assert_eq!(State::new().with_code("4").run().unwrap().int().unwrap(), 4);
        assert_eq!(
            State::new()
                .with_code("=(x, 4), x")
                .run()
                .unwrap()
                .int()
                .unwrap(),
            4
        );
    }
}
