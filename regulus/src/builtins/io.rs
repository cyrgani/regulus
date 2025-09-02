use crate::exception::IoError;
use crate::prelude::*;

functions! {
    /// Takes no arguments and reads from stdin until a newline is entered.
    /// Returns the read input, excluding the newline, as a string.
    "input"(0) => |state, _| {
        let mut input = String::new();
        match state.stdin.read_line(&mut input) {
            // TODO: consider removing this exception and using `.unwrap_or(&input).to_string()` instead
            Ok(_) => Ok(Atom::String(
                input
                    .strip_suffix('\n')
                    .ok_or_else(|| state.raise(IoError, "missing newline after input() call"))?
                    .to_string(),
            )),
            Err(error) => {
                raise!(state, IoError, "Error while reading input: {error}")
            }
        }
    }
    /// Evaluates the given argument and prints it to stdout, without any additional spaces or newline.
    "write"(1) => |state, args| {
        let s = args[0].eval(state)?.to_string();
        state.write_to_stdout(&s);
        Ok(Atom::Null)
    }
}
