use regulus::prelude::{Atom, State};
use std::env;
use std::process::exit;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("error: program file not provided\nusage: cargo run -- PATH",);
        exit(1);
    };

    let mut state = State::new();
    match state.with_source_file(path) {
        Ok(updated) => {
            state = updated;
        }
        Err(err) => {
            eprintln!("reading the file caused an error: {err}");
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
            eprintln!("uncaught exception occured: \n{error}");
            exit(1);
        }
    }
}
