use crate::prelude::*;
use crate::state::Directory;
use std::borrow::Cow;

functions! {
    /// Evaluates all given arguments and returns the atom the last argument evaluated to.
    /// If no arguments are given, `null` is returned.
    ///
    /// Every program is implicitly wrapped in a call to this function.
    ///
    /// This function has an alias: `run`.
    "_"(_) => |state, args| {
        if args.is_empty() {
            Ok(Atom::Null)
        } else {
            for arg in &args[0..args.len() - 1] {
                arg.eval(state)?;
            }
            args[args.len() - 1].eval(state).map(Cow::into_owned)
        }
    }
    /// Assigns the second argument to a variable named like the first argument.
    ///
    /// This function has an alias: `assign`.
    "="(2) => |state, args| {
        let var = args[0].variable("Error during assignment: no variable was given to assign to!", state)?;
        let value = args[1].eval(state)?.into_owned();
        state.storage.insert(var, value);
        Ok(Atom::Null)
    }
    /// Evaluates the first argument as a boolean.
    /// If it evaluates to true, the second argument is evaluated and returned.
    /// If it evaluates to false, the third argument is evaluated and returned instead.
    "ifelse"(3) => |state, args| {
        Ok(if args[0].eval_bool(state)? {
            args[1].eval(state)?
        } else {
            args[2].eval(state)?
        }.into_owned())
    }
    /// Repeatedly evaluates the first argument as a boolean.
    /// If it evaluates to true, the second argument is evaluated and the same steps begin again.
    /// If it evaluates to false, the loop ends and `null` is returned.
    "while"(2) => |state, args| {
        while args[0].eval_bool(state)? {
            args[1].eval(state)?;
        }
        Ok(Atom::Null)
    }
    /// Raises an exception.
    /// The first argument is a string that describes the error kind.
    /// The second argument is a string error message.
    ///
    /// The error kind should be a captialized word.
    /// When displaying the error kind, `Error` will be appended implicitly, so the error kind given
    /// here should not end in `Error`, `Exception` or similar.
    "error"(2) => |state, args| {
        let kind = args[0].eval_string(state)?;
        let msg = args[1].eval_string(state)?;
        Err(state.raise(kind, msg))
    }
    /// Evaluates the given value and returns it.
    /// If an exception occurs while evaluating the argument, the exception is converted into a
    /// string and returned instead.
    "run_or_string_exception"(1) => |state, args| {
        Ok(match args[0].eval(state) {
            Ok(atom) => Cow::into_owned(atom),
            Err(exc) => Atom::String(exc.to_string())
        })
    }
    /// Evaluates the given argument and terminates the program directly.
    /// The program will return the given value as its final result.
    ///
    /// Even if the argument causes an exception, it is returned directly too.
    ///
    /// If `exit` is reached via an `import`-ed module, it will stop the main program too.
    "exit"(1) => |state, args| {
        let value = args[0].eval(state).map(Cow::into_owned);
        state.exit_unwind_value = Some(value);
        Ok(Atom::Null)
    }
    /// Evaluates the given argument as a string, then treats this string as Regulus code and executes it.
    /// Returns the result of that program.
    ///
    /// Variables defined inside the evaluated code are not visible outside of the `eval` invocation.
    ///
    /// TODO: think about imports, test them
    "eval"(1) => |state, args| {
        let code = args[0].eval_string(state)?;
        let mut state = State::new().with_code(code);
        state.file_directory = Directory::FromEval;
        state.run()
    }
    /// Defines a new variable as global and assigns it the given value.
    "global"(2) => |state, args| {
        let var = args[0].variable("`global(2)` expects a variable argument", state)?;
        let atom = args[1].eval(state)?.into_owned();
        state.storage.add_global(var, atom);
        Ok(Atom::Null)
    }
    /// Executes the first argument. If it raises an uncaught exception, runs the second argument.
    ///
    /// If the second argument also throws an exception, it will not be caught by this call and
    /// propagate further.
    ///
    /// Returns `null`.
    ///
    /// TODO: consider instead returning what the first argument evaluates to (if successfull),
    ///  otherwise returning the eval of the second arg.
    "try_except"(2) => |state, args| {
        if args[0].eval(state).is_err() {
            args[1].eval(state)?;
        }
        Ok(Atom::Null)
    }
    // TODO: invent some way for objects to define how they want to be printed.
    // TODO: try then moving this to the STL
    /// Evaluates the given arg and returns a string representation of it.
    /// See the documentation of `string(1)` for a comparison of these two methods.
    /// Note that the exact output format is not yet stable and may change, especially regarding
    /// objects.
    ///
    /// This is identical to the output of `write`.
    "printable"(1) => |state, args| {
        Ok(Atom::String(args[0].eval(state)?.to_string()))
    }
}
