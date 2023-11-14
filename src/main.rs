use newlang::{run, Atom};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if let Some(file_path) = args.get(1) {
        let file = fs::read_to_string(file_path);
        match file {
            Ok(code) => {
                let result = run(&code);
                match result {
                    Ok((atom, _storage)) => match atom {
                        Atom::Null => (),
                        _ => println!("{:?}", atom),
                    },
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        }
    } else {
        eprintln!("Error: No source file was given!");
    }
}
