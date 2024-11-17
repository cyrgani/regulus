use crate::function;
use crate::prelude::*;
use crate::STL_DIRECTORY;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn functions() -> Vec<Function> {
    vec![
        run_fn(),
        assign(),
        r#if(),
        ifelse(),
        r#while(),
        def(),
        def_args(),
        import(),
        error(),
        equals(),
        assert(),
        catch(),
    ]
}

fn run_fn() -> Function {
    Function {
        name: String::from("_"),
        aliases: vec!["run".to_string()],
        argc: None,
        callback: Rc::new(|state, args| {
            if args.is_empty() {
                Ok(Atom::Null)
            } else {
                for arg in &args[0..args.len() - 1] {
                    arg.eval(state)?;
                }
                args[args.len() - 1].eval(state)
            }
        }),
    }
}

function! {
    name: assign,
    aliases: vec!["=".to_string()],
    argc: Some(2),
    callback: |state, args| {
        if let Argument::Variable(var) = &args[0] {
            let val = args[1].eval(state)?;
            state.storage.insert(var.clone(), val);
            Ok(Atom::Null)
        } else {
            Exception::new_err(
                "Error during assignment: no variable was given to assign to!",
                Error::Assign,
            )
        }
    },
}

function! {
    name: r#if,
    argc: Some(2),
    callback: |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?
        } else {
            Atom::Null
        })
    },
}

function! {
    name: ifelse,
    argc: Some(3),
    callback: |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?
        } else {
            args[2].eval(state)?
        })
    },
}

function! {
    name: r#while,
    argc: Some(2),
    callback: |state, args| {
        while args[0].eval(state)?.bool()? {
            args[1].eval(state)?;
        }
        Ok(Atom::Null)
    },
}

function! {
    name: def,
    argc: Some(3),
    callback: |state, args| {
        if let Argument::Variable(var) = &args[0] {
            if let Argument::FunctionCall(arg_call) = &args[1] {
                if let Argument::FunctionCall(inner) = &args[2] {
                    let body = inner.clone();
                    let function_arg_names = arg_call
                        .args
                        .iter()
                        .cloned()
                        .map(|fn_arg| match fn_arg {
                            Argument::Variable(fn_arg) => Ok(fn_arg),
                            _ => Exception::new_err(
                                "Error during definition: invalid args were given!",
                                Error::Assign,
                            ),
                        })
                        .collect::<Result<Vec<String>, Exception>>()?;

                    let function = Function {
                        aliases: vec![],
                        name: var.clone(),
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
                    Exception::new_err(
                        "Error during definition: no valid function body was given!",
                        Error::Assign,
                    )
                }
            } else {
                Exception::new_err(
                    "Error during definition: no valid argument was given!",
                    Error::Assign,
                )
            }
        } else {
            Exception::new_err(
                "Error during definition: no valid variable was given to define to!",
                Error::Assign,
            )
        }
    },
}

fn def_args() -> Function {
    Function::new(
        &["@", "args"],
        None,
        Rc::new(|_state, _args| {
            unreachable!("this function should never get evaluated and only be used without evaluation by `def`s internals!")
        }),
    )
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

function! {
    name: import,
    argc: Some(1),
    callback: |state, args| {
        let name = args[0].eval(state)?.string()?;
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Exception::new_err(
                format!("invalid characters in import name `{name}`"),
                Error::Import,
            );
        }

        // lookup order:
        // 1. look inside the programs current directory
        // 2. look in the global stl directory
        let mut source = None;

        for item in read_dir_files(&state.file_directory).chain(read_dir_files(&STL_DIRECTORY))
        {
            if *dbg!(item.file_name()) == *dbg!(format!("{name}.prog")) {
                if let Ok(file_content) = fs::read_to_string(item.path()) {
                    source = Some(file_content);
                    break;
                }
            }
        }
        let Some(code) = source else {
            return Exception::new_err(
                format!("failed to find file for importing `{name}`"),
                Error::Import,
            );
        };

        let (atom, imported_state) = run(&code, &state.file_directory, None)?;

        for (k, v) in imported_state.storage {
            state.storage.insert(k, v);
        }
        Ok(atom)
    },
}

function! {
    name: error,
    argc: Some(1),
    callback: |state, args| {
        Exception::new_err(args[0].eval(state)?.string()?, Error::UserRaised)
    },
}

function! {
    name: catch,
    argc: Some(1),
    callback: |state, args| {
        Ok(args[0]
            .eval(state)
            .unwrap_or_else(|exc| Atom::String(exc.to_string())))
    },
}

fn equals() -> Function {
    Function {
        name: String::from("=="),
        aliases: vec!["equals".to_string()],
        argc: Some(2),
        callback: Rc::new(|state, args| {
            Ok(Atom::Bool(args[0].eval(state)? == args[1].eval(state)?))
        }),
    }
}

function! {
    name: assert,
    argc: Some(1),
    callback: |state, args| {
        if args[0].eval(state)?.bool()? {
            Ok(Atom::Null)
        } else {
            Exception::new_err("Assertion failed!", Error::Assertion)
        }
    },
}
