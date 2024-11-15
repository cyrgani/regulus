use crate::prelude::*;
use crate::stdio::{self, STDIN, STDOUT};
use std::io::Write;

fn write_to_stdout(msg: &str) {
    stdio::get_mut(&STDOUT).write_all(msg.as_bytes()).unwrap();
}

pub fn functions() -> Vec<Function> {
    vec![print(), input(), debug()]
}

fn print() -> Function {
    Function {
        aliases: vec![],
        name: String::from("print"),
        argc: None,
        callback: Rc::new(|storage, args| {
            for arg in args {
                write_to_stdout(&format!("{} ", arg.eval(storage)?));
            }
            write_to_stdout("\n");
            Ok(Atom::Null)
        }),
    }
}

fn input() -> Function {
    Function {
        aliases: vec![],
        name: String::from("input"),
        argc: Some(0),
        callback: Rc::new(|_, _| {
            let mut input = String::new();
            match stdio::get_mut(&STDIN).read_line(&mut input) {
                Ok(_) => Ok(Atom::String(input)),
                Err(error) => {
                    Exception::new_err(format!("Error while reading input: {error}"), Error::Io)
                }
            }
        }),
    }
}

fn debug() -> Function {
    Function {
        aliases: vec![],
        name: String::from("debug"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            write_to_stdout(&format!("Debug: {:?}\n", args[0].eval(storage)?));
            Ok(Atom::Null)
        }),
    }
}
