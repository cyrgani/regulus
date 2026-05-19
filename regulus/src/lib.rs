#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::dbg_macro
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::needless_pass_by_value,
    clippy::missing_const_for_fn,
    clippy::type_complexity
)]

mod argument;
mod atom;
mod exception;
mod function;
mod list;
mod macros;
mod optimizations;
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
        exception::{Exception, Result},
        function::{Function, FunctionBody, FunctionCall},
        functions,
        list::List,
        parsing::{Position, Span},
        raise, run, run_file,
        state::{State, Storage, WriteHandle},
    };
}

use crate::{atom::Atom, exception::Result, state::State};
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub(crate) fn no_path() -> Rc<PathBuf> {
    Rc::new(PathBuf::new())
}

/// Files need to end with this extension to be considered by import resolution.
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

    #[test]
    fn state_reuse_exit() {
        let mut state = State::new().with_code("exit(2)");
        assert_eq!(state.run().unwrap(), Atom::Int(2));
        state = state.with_code("exit(1)");
        assert_eq!(state.run().unwrap(), Atom::Int(1));
        state = state.with_code("3");
        assert_eq!(state.run().unwrap(), Atom::Int(3));
    }

    #[expect(clippy::neg_cmp_op_on_partial_ord, reason = "that's what the test is about")]
    #[test]
    fn atom_not_ord() {
        assert!(!(Atom::Null < Atom::Int(2)));
        assert!(!(Atom::Null == Atom::Int(2)));
        assert!(!(Atom::Null > Atom::Int(2)));
    }
}
