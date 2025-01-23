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
mod macros;
mod parsing;
mod state;

mod stdlib;

pub mod prelude;

use crate::{
    atom::Atom,
    exception::Result,
    parsing::token::extract,
    parsing::{build_program, tokenize, validate_tokens},
    state::State,
};
use std::path::Path;

macro_rules! return_err {
    ($val: expr, $state: expr) => {
        match $val {
            Ok(ok) => ok,
            Err(err) => return (Err(err), $state),
        }
    };
}

pub fn run(code: &str, dir: impl AsRef<Path>, start_state: Option<State>) -> (Result<Atom>, State) {
    let mut state = start_state.unwrap_or_else(|| State::initial(dir));
    let tokens = return_err!(tokenize(code), state);
    return_err!(validate_tokens(&tokens), state);
    for token in &tokens {
        println!("token `{:?}` and span ``{:?}", token.data, token.indices);
    }

    for token in &tokens {
        /*println!(
            "token `{:?}` has expansion `{}`",
            token.data,
            extract(code, token.indices.clone()).unwrap()
        );*/
        print!("\"{}\",", extract(code, token.indices.clone()).unwrap());
    }

    let program = return_err!(build_program(&tokens, "_"), state);

    let result = return_err!(program.eval(&mut state), state);
    (Ok(result), state)
}
