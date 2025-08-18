use colored::Colorize;
use regulus::prelude::{Atom, State};
use std::env;
use std::process::exit;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!(
            "{} program file not provided\n{} cargo run -- PATH",
            "error:".red(),
            "usage:".underline().bold()
        );
        exit(1);
    };

    let mut state = State::new();
    match state.with_source_file(path) {
        Ok(updated) => {
            state = updated;
        }
        Err(err) => {
            eprintln!(
                "{}",
                format!("Reading the file caused an error: {err}").red()
            );
            exit(1);
        }
    }

    match state.run() {
        Ok(atom) => {
            if atom != Atom::Null {
                println!("{atom}");
            }
        }
        Err(error) => {
            eprintln!("{}", error.to_string().red());
            exit(1);
        }
    }
}
