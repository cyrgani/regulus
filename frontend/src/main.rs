use clap::Parser;
use colored::Colorize;
use regulus::prelude::*;
//use regulus::{FUNCTION_CLONE_COUNT, LIST_CLONE_COUNT, OBJECT_CLONE_COUNT, STRING_CLONE_COUNT};
use std::path::PathBuf;
use std::process::exit;
//use std::time::Instant;

/// An interpreter for the Regulus language.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the program
    path: PathBuf,

    // /// TODO: activate debug mode
    //#[arg(short, long, default_value_t = false)]
    //debug: bool,
    /// Show the final storage
    #[arg(short = 'S', long, default_value_t = false)]
    dump_storage: bool,
    // /// TODO: Show colored output
    //#[arg(short, long, default_value_t = true)]
    //colored: bool,
}

fn main() {
    let args = Args::parse();

    //let t = Instant::now();
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
                _ => println!("{atom:?}"),
            };
            if args.dump_storage {
                println!("{:?}", state.storage.data)
            }
        }
        Err(error) => {
            eprintln!(
                "{}",
                format!("The program caused an error: {}", error.display(&state)).red()
            );
            exit(1);
        }
    }
    /*dbg!(&FUNCTION_CLONE_COUNT);
    dbg!(&OBJECT_CLONE_COUNT);
    dbg!(&STRING_CLONE_COUNT);
    dbg!(&LIST_CLONE_COUNT);
    dbg!(t.elapsed());*/
}
