use crate::prelude::*;

functions! {
    /// Evaluates all given arguments and prints them to stdout.
    /// All arguments are separated with a single space.
    /// After all arguments have been printed, a newline is also printed.
    /// Returns `null`.
    ///
    /// If you need more precise control over the output, use `write` instead.
    "print"(_) => |state, args| {
        for arg in args {
            let arg_val = arg.eval(state)?;
            let s = format!("{arg_val} ");
            state.write_to_stdout(&s);
        }
        state.write_to_stdout("\n");
        Ok(Atom::Null)
    }
    /// Takes no arguments and reads from stdin until a newline is entered.
    /// Returns the read input, excluding the newline, as a string.
    "input"(0) => |state, _| {
        let mut input = String::new();
        match state.stdin().read_line(&mut input) {
            // TODO: consider removing this exception and using `.unwrap_or(&input).to_string()` instead
            Ok(_) => Ok(Atom::String(input.strip_suffix('\n').ok_or_else(|| Exception::new("missing newline after input() call", Error::Io))?.to_string())),
            Err(error) => {
                raise!(Error::Io, "Error while reading input: {error}")
            }
        }
    }
    /// Prints the debug representation of the given argument to stdout, followed by a newline.
    ///
    /// NOTE: the output format of this method is unstable.
    /// NOTE: this method may be removed in the future.
    "__builtin_rust_debug"(1) => |state, args| {
        let arg_val = args[0].eval(state)?;
        let s = format!("{arg_val:?}\n");
        state.write_to_stdout(&s);
        Ok(Atom::Null)
    }
    /// Evaluates the given argument and prints it to stdout, without any additional spaces or newline.
    "write"(1) => |state, args| {
        let s = args[0].eval(state)?.to_string();
        state.write_to_stdout(&s);
        Ok(Atom::Null)
    }
}
