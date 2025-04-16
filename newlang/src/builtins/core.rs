use crate::prelude::*;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::rc::Rc;

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
        if let ArgumentData::Variable(var) = &args[0].data {
            let val = args[1].eval(state)?;
            state.storage.insert(var.clone(), val);
            Ok(Atom::Null)
        } else {
            raise!(
                Error::Assign,
                "Error during assignment: no variable was given to assign to!",
            )
        }
    }
    /// Evaluates the first argument as a boolean.
    /// If it evaluates to true, the second argument is evaluated and returned.
    /// If it evaluates to false, the second argument is ignored and `null` is returned.
    "if"(2) => |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?
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
        })
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
        if args.len() < 2 {
            return raise!(
                Error::Argument,
                "too few arguments passed to `def`: expected at least 2, found {}", args.len()
            );
        }
        if let ArgumentData::Variable(var) = &args[0].data {
            if let ArgumentData::FunctionCall(inner) = &args.last().unwrap().data {
                let body = inner.clone();
                let function_arg_names = args[1..args.len() - 1]
                    .iter()
                    .cloned()
                    .map(|fn_arg| match fn_arg.data {
                        ArgumentData::Variable(fn_arg) => Ok(fn_arg),
                        _ => raise!(
                            Error::Assign,
                            "Error during definition: invalid args were given!",
                        ),
                    })
                    .collect::<Result<Vec<String>>>()?;

                let function = Function {
                    doc: String::new(),
                    argc: Some(function_arg_names.len()),
                    callback: Rc::new(move |state, args| {
                        // a function call should have its own scope and not leak variables
                        let old_storage = state.storage.clone();

                        for (idx, arg) in function_arg_names.iter().enumerate() {
                            let arg_result = args[idx].eval(state)?;
                            state.storage.insert(arg.clone(), arg_result);
                        }

                        let function_result = body.eval(state);
                        state.storage = old_storage;
                        function_result
                    }),
                };

                state.storage.insert(var.clone(), Atom::Function(function));
                Ok(Atom::Null)
            } else {
                raise!(
                    Error::Assign,
                    "Error during definition: no valid function body was given!",
                )
            }
        } else {
            raise!(
                Error::Assign,
                "Error during definition: no valid variable was given to define to!",
            )
        }
    }
    /// Imports a file, either from the stl or the local directory.
    /// TODO document the exact algorithm and hierarchy more clearly, also the behavior of `=`
    "import"(1) => |state, args| {
        let name = args[0].eval(state)?.string()?;
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return raise!(
                Error::Import,
                "invalid characters in import name `{name}`",
            );
        }

        // lookup order:
        // 1. look inside the programs current directory
        // 2. look in the global stl directory
        let mut source = None;

        for item in read_dir_files(&state.file_directory).chain(read_dir_files(&state.stl_path))
        {
            if *item.file_name() == *format!("{name}.prog") {
                if let Ok(file_content) = fs::read_to_string(item.path()) {
                    source = Some(file_content);
                    break;
                }
            }
        }
        let Some(code) = source else {
            return raise!(
                Error::Import,
                "failed to find file for importing `{name}`",
            );
        };

        let (atom, imported_state) = crate::run_with_options(&code, &state.file_directory, None, &state.stl_path);
        let atom = atom?;

        for (k, v) in imported_state.storage {
            state.storage.insert(k, v);
        }
        Ok(atom)
    }
    /// Raises an exception of the kind `UserRaised` with the given string message.
    "error"(1) => |state, args| {
        Err(Exception::new(args[0].eval(state)?.string()?, Error::UserRaised))
    }
    /// Evaluates the given value and returns it.
    /// If an exception occurs while evaluating the argument, the exception is converted into a
    /// string and returned instead.
    "catch"(1) => |state, args| {
        Ok(args[0]
            .eval(state)
            .unwrap_or_else(|exc| Atom::String(exc.to_string())))
    }
    /// Evaluates both arguments and returns whether they are equal.
    /// TODO: define this behavior in edge cases and document it
    "=="(2) => |state, args| {
        Ok(Atom::Bool(args[0].eval(state)? == args[1].eval(state)?))
    }
    /// Evaluates the argument as a boolean and returns `null` if it is true.
    /// If it is false, raise an exception of the `Assertion` kind.
    "assert"(1) => |state, args| {
        if args[0].eval(state)?.bool()? {
            Ok(Atom::Null)
        } else {
            raise!(Error::Assertion, "Assertion failed!")
        }
    }
    /// Evaluates both arguments and compares then, returning `null` if they are equal.
    /// If not, raise an exception of the `Assertion` kind with a message containing both values.
    "assert_eq"(2) => |state, args| {
        let lhs = args[0].eval(state)?;
        let rhs = args[1].eval(state)?;
        if lhs == rhs {
            Ok(Atom::Null)
        } else {
            raise!(
                Error::Assertion,
                "Equality assertion failed! lhs: `{lhs}`, rhs: `{rhs}`!"
            )
        }
    }
}

fn read_dir_files(path: impl AsRef<Path>) -> impl Iterator<Item = DirEntry> {
    fs::read_dir(&path)
        .unwrap_or_else(|err| {
            panic!(
                "error when reading directory `{}`: {err}",
                path.as_ref().display()
            )
        })
        .flatten()
}
