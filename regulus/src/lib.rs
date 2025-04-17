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
use std::path::Path;

pub const FILE_EXTENSION: &str = "re";

pub(crate) const STL_DIR: &str = "stdlib";

pub fn run(code: &str, dir: impl AsRef<Path>) -> (Result<Atom>, State) {
    run_with_options(code, dir, None, STL_DIR)
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
