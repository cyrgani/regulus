use newlang::run;
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if let Some(file_path) = args.get(1) {
        let file = fs::read_to_string(file_path);
        match file {
            Ok(code) => {
                let result = run(&code);
                match result {
                    Ok(atom) => println!("{:?}", atom),
                    Err(error) => eprintln!("{}", error),
                }
            }
            Err(error) => eprintln!("{}", error),
        }
    } else {
        eprintln!("No souce file was given!");
    }
}
