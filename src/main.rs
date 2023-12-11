use clap::Parser;
use colored::Colorize;
use newlang::prelude::*;
use std::fs;

/// An interpreter for the language NAMEHERE
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the program
    path: String,

    /// TODO: activate debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// TODO: Show the final storage
    #[arg(short = 'S', long, default_value_t = false)]
    dump_storage: bool,

    /// TODO: Show colored output
    #[arg(short, long, default_value_t = true)]
    colored: bool,
}

fn main() {
    let args = Args::parse();

    let file = fs::read_to_string(args.path);
    match file {
        Ok(code) => {
            let result = run(&code, None);
            match result {
                Ok((atom, storage)) => {
                    match atom {
                        Atom::Null => (),
                        _ => println!("{atom:?}"),
                    };
                    if args.dump_storage {
                        println!("{storage:?}")
                    }
                }
                Err(error) => {
                    eprintln!("{}", format!("The program caused an error: {error}").red());
                }
            }
        }
        Err(error) => eprintln!(
            "{}",
            format!("Reading the file caused an error: {error}").red()
        ),
    }
}
