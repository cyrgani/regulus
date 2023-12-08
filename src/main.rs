use colored::Colorize;
use newlang::prelude::*;
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if let Some(file_path) = args.get(1) {
        let file = fs::read_to_string(file_path);
        match file {
            Ok(code) => {
                let result = run(&code, None);
                match result {
                    Ok((atom, _storage)) => match atom {
                        Atom::Null => (),
                        _ => println!("{atom:?}"),
                    },
                    Err(error) => {
                        eprintln!("{}", format!("The program caused an error: {error}").red())
                    }
                }
            }
            Err(error) => eprintln!(
                "{}",
                format!("Reading the file caused an error: {error}").red()
            ),
        }
    } else {
        eprintln!("{}", "Error: No source file was given!".red());
    }
}
