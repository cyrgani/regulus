use crate::prelude::*;
use crate::state::Directory;

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
            args[args.len() - 1].eval(state)
        }
    }
    /// Assigns the second argument to a variable named like the first argument.
    ///
    /// This function has an alias: `assign`.
    "="(2) => |state, args| {
        let var = args[0].variable("invalid assignment: tried to assign to a non-variable", state)?;
        let value = args[1].eval(state)?;
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
        })
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
        let kind = args[0].eval_as_string(state)?;
        let msg = args[1].eval_as_string(state)?;
        Err(state.raise(kind, msg))
    }
    /// Evaluates the given argument and terminates the program directly.
    /// The program will return the given value as its final result.
    ///
    /// If the argument causes an exception, it is not returned directly
    /// and the exception may still be caught with `try_except`.
    ///
    /// If `exit` is reached via an `import`-ed module, it will stop the main program too.
    "exit"(1) => |state, args| {
        let value = args[0].eval(state)?;
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
        let code = args[0].eval_as_string(state)?;
        let mut eval_state = State::new().with_code(code);
        eval_state.file_directory = Directory::FromEval;
        eval_state.run()
    }
    /// Defines a new variable as global and assigns it the given value.
    "global"(2) => |state, args| {
        let var = args[0].variable("`global(2)` expects a variable argument", state)?;
        let atom = args[1].eval(state)?;
        state.storage.add_global(var, atom);
        Ok(Atom::Null)
    }
    /// Executes the first argument. If it raises an uncaught exception, runs the second argument.
    /// This optionally takes an identifier as a third argument. If it is passed, it will be
    /// assigned the stringified exception message.
    ///
    /// If the second argument also throws an exception, it will not be caught by this call and
    /// propagate further.
    ///
    /// This returns what the first argument evaluates to (if successful),
    /// otherwise it returns the eval of the second arg.
    "try_except"(_) => |state, args| {
        match args {
            [body, fallback] => {
                match body.eval(state) {
                    Ok(val) => Ok(val),
                    Err(_) => fallback.eval(state),
                }
            }
            [body, fallback, exception_var] => {
                let exc_var = exception_var.variable("invalid exception variable given to `try_except`", state)?;
                match body.eval(state) {
                    Ok(val) => Ok(val),
                    Err(e) => {
                        state.storage.insert(exc_var, Atom::new_string(&e.to_string()));
                        fallback.eval(state)
                    },
                }
            }
            _ => raise!(
                state,
                "Argument",
                "invalid number of args for `try_except`: should be 2 or 3, was {}",
                args.len()
            )
        }
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
        Ok(Atom::new_string(&args[0].eval(state)?.to_string()))
    }
    /// Iterates over the given list elements.
    /// The first argument is the list, the second the loop variable name for each element and the
    /// third is the body that will be run for each of these elements.
    /// Afterwards, `null` is returned.
    // TODO: argument order of seq and loop var is confusing
    "for_in"(3) => |state, args| {
        let v = args[0].eval_list(state)?;
        let loop_var = args[1].variable("invalid loop variable given to `for_in`", state)?;
        let loop_body = &args[2];
        for el in v.iter() {
            state.storage.insert(loop_var, el.clone());
            loop_body.eval(state)?;
        }

        Ok(Atom::Null)
    }
    /// Returns the documentation string for a function.
    "doc"(1) => |state, args| {
        Ok(Atom::new_string(args[0].eval_function(state)?.doc()))
    }
    /// Returns the argument count for a function, or `null` if it has none.
    "argc"(1) => |state, args| {
        Ok(if let Some(argc) = args[0].eval_function(state)?.argc() {
            Atom::int_from_rust_int(argc, state)?
        } else {
            Atom::Null
        })
    }
    /// Takes no arguments and reads from stdin until a newline is entered.
    /// Returns the read input, excluding the newline, as a string.
    "input"(0) => |state, _| {
        let mut input = String::new();
        match state.stdin.read_line(&mut input) {
            Ok(_) => Ok(Atom::new_string(
                input.strip_suffix('\n').unwrap_or(&input)
            )),
            Err(error) => {
                raise!(state, "Io", "error while reading input: {error}")
            }
        }
    }
    /// Evaluates the given argument and prints it to stdout, without any additional spaces or newline.
    "write"(1) => |state, args| {
        let s = args[0].eval(state)?.to_string();
        state.write_to_stdout(&s);
        Ok(Atom::Null)
    }
    /// Evaluates both arguments as booleans and performs short-circuiting OR on them.
    "||"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval_bool(state)? || args[1].eval_bool(state)?
    ))
    /// Evaluates both arguments as booleans and performs short-circuiting AND on them.
    "&&"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval_bool(state)? && args[1].eval_bool(state)?
    ))
}
