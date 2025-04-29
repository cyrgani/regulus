use clap::Parser;
use colored::Colorize;
use regulus::prelude::*;
use std::path::PathBuf;
use std::process::exit;

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

    let mut runner = Runner::new();
    match runner.file(args.path) {
        Ok(updated) => {
            runner = updated;
        }
        Err(err) => {
            eprintln!(
                "{}",
                format!("Reading the file caused an error: {err}").red()
            );
            exit(1);
        }
    }

    let result = runner.run();
    match result {
        (Ok(atom), state) => {
            match atom {
                Atom::Null => (),
                _ => println!("{atom:?}"),
            };
            if args.dump_storage {
                println!("{:?}", state.storage.data)
            }
        }
        (Err(error), _) => {
            eprintln!("{}", format!("The program caused an error: {error}").red());
            exit(1);
        }
    }
}
