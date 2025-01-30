use crate::prelude::*;

fn write_to_stdout(state: &mut State, msg: &str) {
    state.stdout.write_all(msg.as_bytes());
}

functions! {
    /// Evaluates all given arguments and prints them to stdout.
    /// All arguments are separated with a single space.
    /// After all arguments have been printed, a newline is also printed.
    /// Returns `null`.
    "print"(_) => |state, args| {
        for arg in args {
            let arg_val = arg.eval(state)?;
            write_to_stdout(state, &format!("{arg_val} "));
        }
        write_to_stdout(state, "\n");
        Ok(Atom::Null)
    }
    /// Takes no arguments and reads from stdin until a newline is entered.
    /// Returns the read input, including the newline, as a string.
    "input"(0) => |state, _| {
        let mut input = String::new();
        match state.stdin.read_line(&mut input) {
            Ok(_) => Ok(Atom::String(input)),
            Err(error) => {
                raise!(Error::Io, "Error while reading input: {error}")
            }
        }
    }
    /// Prints the debug representation of the given argument to stdout, followed by a newline.
    /// 
    /// NOTE: the output format of this method is unstable.
    /// NOTE: this method may be removed in the future.
    "debug"(1) => |state, args| {
        let arg_val = args[0].eval(state)?;
        write_to_stdout(state, &format!("{arg_val:?}\n"));
        Ok(Atom::Null)
    }
}
