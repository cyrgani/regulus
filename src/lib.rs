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

pub const STL_DIRECTORY: &str = "./src/stdlib/library";

mod argument;
mod atom;
mod exception;
mod function;
mod parsing;
mod state;
mod utils;

mod stdlib;

pub mod prelude;

use crate::{
    atom::Atom,
    exception::ProgResult,
    parsing::{build_program, tokenize, validate_tokens},
    state::State,
};
use std::path::Path;

pub fn run(
    code: &str,
    dir: impl AsRef<Path>,
    start_state: Option<State>,
) -> ProgResult<(Atom, State)> {
    let tokens = tokenize(code);

    validate_tokens(&tokens)?;

    let program = build_program(&tokens, "_")?;

    let mut state = start_state.unwrap_or_else(|| state::State::initial(dir));

    let result = program.eval(&mut state)?;
    Ok((result, state))
}
