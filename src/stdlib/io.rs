use crate::prelude::*;

fn write_to_stdout(state: &mut State, msg: &str) {
    state.stdout.write_all(msg.as_bytes());
}

export! {
    print, input, debug,
}

function! {
    name: print,
    argc: None,
    callback: |state, args| {
        for arg in args {
            let arg_val = arg.eval(state)?;
            write_to_stdout(state, &format!("{arg_val} "));
        }
        write_to_stdout(state, "\n");
        Ok(Atom::Null)
    },
}

function! {
    name: input,
    argc: Some(0),
    callback: |state, _| {
        let mut input = String::new();
        match state.stdin.read_line(&mut input) {
            Ok(_) => Ok(Atom::String(input)),
            Err(error) => {
                Exception::new_err(format!("Error while reading input: {error}"), Error::Io)
            }
        }
    },
}

function! {
    name: debug,
    argc: Some(1),
    callback: |state, args| {
        let arg_val = args[0].eval(state)?;
        write_to_stdout(state, &format!("Debug: {arg_val:?}\n"));
        Ok(Atom::Null)
    },
}
