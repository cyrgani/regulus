use crate::prelude::*;
use crate::state::State;

fn write_to_stdout(state: &mut State, msg: &str) {
    state.stdout.write_all(msg.as_bytes());
}

pub fn functions() -> Vec<Function> {
    vec![print(), input(), debug()]
}

fn print() -> Function {
    Function {
        aliases: vec![],
        name: String::from("print"),
        argc: None,
        callback: Rc::new(|state, args| {
            for arg in args {
                let arg_val = arg.eval(state)?;
                write_to_stdout(state, &format!("{arg_val} "));
            }
            write_to_stdout(state, "\n");
            Ok(Atom::Null)
        }),
    }
}

fn input() -> Function {
    Function {
        aliases: vec![],
        name: String::from("input"),
        argc: Some(0),
        callback: Rc::new(|state, _| {
            let mut input = String::new();
            match state.stdin.read_line(&mut input) {
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
        callback: Rc::new(|state, args| {
            let arg_val = args[0].eval(state)?;
            write_to_stdout(state, &format!("Debug: {arg_val:?}\n"));
            Ok(Atom::Null)
        }),
    }
}
