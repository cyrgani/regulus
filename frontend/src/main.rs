use clap::Parser;
use colored::Colorize;
use regulus::prelude::{Atom, State};
use std::path::PathBuf;
use std::process::exit;

/// An interpreter for the Regulus language.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the program
    path: PathBuf,

    /// Show the final storage
    #[arg(short = 'S', long, default_value_t = false)]
    dump_storage: bool,
}

fn main() {
    let args = Args::parse();

    let mut state = State::new();
    match state.with_source_file(args.path) {
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
            match atom {
                Atom::Null => (),
                _ => println!("{atom}"),
            };
            if args.dump_storage {
                println!("{:?}", state.storage.data)
            }
        }
        Err(error) => {
            eprintln!("{}", error.to_string().red());
            exit(1);
        }
    }
}
