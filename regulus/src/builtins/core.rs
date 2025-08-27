use crate::exception::ArgumentError;
use crate::exception::ImportError;
use crate::interned_stdlib::INTERNED_STL;
use crate::prelude::*;
use crate::state::Directory;
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

fn define_function(body: &Argument, fn_args: &[Argument], state: &State) -> Result<Atom> {
    let body = body.clone();
    let function_arg_names = fn_args
        .iter()
        .map(|fn_arg| {
            fn_arg
                .variable("Error during definition: invalid args were given!", state)
                .cloned()
        })
        .collect::<Result<Vec<_>>>()?;

    let function = Function::new(
        state.current_doc_comment.as_ref().unwrap(),
        Some(function_arg_names.len()),
        Box::new(move |state, args| {
            // a function call should have its own scope and not leak variables
            // except for globals

            // TODO:
            //  this cloning of the whole storage is extremely inefficient
            //  a better idea would be a "tagged" storage (??)
            //  or: create a new empty storage, put all redefined vars in the function into it, but
            //      allow reading from both new and then old in that order
            //      problem: `body.eval(state);` can only take one state, not two
            let mut old_storage_data = state.storage.data.clone();

            // prevent arguments from overwriting each other, ex. f(a,b) calls f(b,a)
            let mut arg_values = Vec::with_capacity(args.len());

            for (idx, arg) in function_arg_names.iter().enumerate() {
                let arg_result = args[idx].eval(state)?.into_owned();
                arg_values.push((arg.clone(), arg_result));
            }

            for (name, value) in arg_values {
                state.storage.insert(name, value);
            }

            let function_result = body.eval(state).map(Cow::into_owned);

            old_storage_data.extend(state.storage.global_items());

            state.storage.data = old_storage_data;
            function_result
        }),
    );

    Ok(Atom::Function(function))
}

/// Returns:
/// * `None` if the resolution in the given directory failed,
/// * `Some(path)` if the code was found outside the interned STL,
fn try_resolve_import_in_dir(name: &str, dir_path: &Path) -> Option<PathBuf> {
    let paths = fs::read_dir(dir_path)
        .unwrap_or_else(|err| {
            panic!(
                "error when reading directory `{}`: {err}",
                dir_path.display()
            )
        })
        .flatten();
    for item in paths {
        if *item.file_name() == *format!("{name}.{FILE_EXTENSION}") {
            return Some(item.path());
        }
    }
    None
}

fn try_resolve_import_in_stl(name: &str) -> Option<String> {
    INTERNED_STL.get(name).map(ToString::to_string)
}

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
    /// If it evaluates to false, the second argument is ignored and `null` is returned.
    "if"(2) => |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?.into_owned()
        } else {
            Atom::Null
        })
    }
    /// Evaluates the first argument as a boolean.
    /// If it evaluates to true, the second argument is evaluated and returned.
    /// If it evaluates to false, the third argument is evaluated and returned instead.
    "ifelse"(3) => |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?
        } else {
            args[2].eval(state)?
        }.into_owned())
    }
    /// Repeatedly evaluates the first argument as a boolean.
    /// If it evaluates to true, the second argument is evaluated and the same steps begin again.
    /// If it evaluates to false, the loop ends and `null` is returned.
    "while"(2) => |state, args| {
        while args[0].eval(state)?.bool()? {
            args[1].eval(state)?;
        }
        Ok(Atom::Null)
    }
    /// Defines a new function.
    /// The first argument is the function identifier and the last argument is the function body.
    /// All arguments in between are the names of the function arguments that can be accessed in
    /// the function body.
    /// Values defined in the function are scoped and cannot be accessed outside of the function body.
    "def"(_) => |state, args| {
        let [var, fn_args @ .., body] = args else {
            raise!(
                state,
                ArgumentError,
                "too few arguments passed to `def`: expected at least 2, found {}", args.len()
            );
        };
        let var = var.variable("Error during function definition: no valid variable was given to define to!", state)?;

        state.storage.insert(var, define_function(body, fn_args, state)?);
        Ok(Atom::Null)
    }
    /// Creates a new function and returns it.
    ///
    /// The last argument is the function body.
    /// All arguments before are the names of the function arguments that can be accessed in
    /// the function body.
    /// Values defined in the function are scoped and cannot be accessed outside of the function body.
    "fn"(_) => |state, args| {
        let Some((body, fn_args)) = args.split_last() else {
            raise!(state, ArgumentError, "`fn` invocation is missing body");
        };
        define_function(body, fn_args, state)
    }
    /// Imports a file, either from the stl or the local directory.
    /// TODO document the exact algorithm and hierarchy more clearly, also the return value of this function
    "import"(1) => |state, args| {
        let name = args[0].variable("`import` argument must be a variable, string syntax was removed", state)?;
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            raise!(
                state,
                ImportError,
                "invalid characters in import name `{name}`, only a-Z, 0-9 and _ are allowed",
            );
        }

        // lookup order:
        // 1. look inside the programs current directory
        // 2. look in the global stl directory
        let mut import_state = State::new();
        let mut found = false;
        if let Directory::Regular(dir_path) = &state.file_directory {
            if let Some(path) = try_resolve_import_in_dir(name, dir_path) {
                import_state = import_state.with_source_file(path).unwrap();
                found = true;
            }
        }
        if !found {
            if let Some(code) = try_resolve_import_in_stl(name) {
                import_state = import_state.with_code(code);
                import_state.set_current_file_path(format!("<stl:{name}>"));
                found = true;
            }
        }
        if !found {
            raise!(state, ImportError, "failed to find file for importing `{name}`");
        }

        import_state.storage.global_idents.clone_from(&state.storage.global_idents);
        import_state.storage.data.extend(state.storage.global_items());
        let atom = import_state.run();

        if let Some(exit_unwind_value) = import_state.exit_unwind_value {
            state.exit_unwind_value = Some(exit_unwind_value);
            return Ok(Atom::Null);
        }
        let atom = atom?;

        for (k, v) in import_state.storage.data {
            state.storage.insert(k, v);
        }
        state.storage.global_idents = import_state.storage.global_idents;
        Ok(atom)
    }
    /// Raises an exception.
    /// The first argument is a string that describes the error kind.
    /// The second argument is a string error message.
    ///
    /// The error kind should be a captialized word.
    /// When displaying the error kind, `Error` will be appended implicitly, so the error kind given
    /// here should not end in `Error`, `Exception` or similar.
    "error"(2) => |state, args| {
        let kind = args[0].eval(state)?.string()?;
        let msg = args[1].eval(state)?.string()?;
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
    /// Evaluates both arguments and returns whether they are equal.
    /// TODO: define this behavior in edge cases and document it
    "__builtin_atom_eq"(2) => |state, args| {
        Ok(Atom::Bool(args[0].eval(state)?.into_owned() == *args[1].eval(state)?))
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
        let code = args[0].eval(state)?.string()?;
        let mut state = State::new().with_code(code);
        state.file_directory = Directory::FromEval;
        state.run()
    }
    /// Marks a variable identifier as global.
    ///
    /// This does not require the identifier to be defined at this time.
    "global"(1) => |state, args| {
        let var = args[0].variable("`global(1)` expects a variable argument", state)?;
        state.storage.global_idents.insert(var.clone());
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
}
