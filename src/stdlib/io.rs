use crate::prelude::*;
use std::io;

pub fn functions() -> Vec<Function> {
    vec![print(), input(), debug()]
}

fn print() -> Function {
    Function {
        aliases: vec![],
        name: String::from("print"),
        argc: None,
        callback: Rc::new(|program, storage, args| {
            for arg in args {
                print!("{} ", arg.eval(program, storage)?.format())
            }
            println!();
            Ok(Atom::Null)
        }),
    }
}

fn input() -> Function {
    Function {
        aliases: vec![],
        name: String::from("input"),
        argc: Some(0),
        callback: Rc::new(|_, _, _| {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => Ok(Atom::String(input)),
                Err(error) => Err(Exception {
                    msg: format!("Error while reading input: {}", error),
                    error: Error::Io,
                }),
            }
        }),
    }
}

fn debug() -> Function {
    Function {
        aliases: vec![],
        name: String::from("debug"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            println!("Debug: {:?}", args[0].eval(program, storage)?);
            Ok(Atom::Null)
        }),
    }
}
