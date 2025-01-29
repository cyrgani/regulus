use crate::prelude::{self, *};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::rc::Rc;

const STL_DIRECTORY: &str = "../stdlib";

functions! {
    _(_) => |state, args| {
        if args.is_empty() {
            Ok(Atom::Null)
        } else {
            for arg in &args[0..args.len() - 1] {
                arg.eval(state)?;
            }
            args[args.len() - 1].eval(state)
        }
    }
    "="(2) => |state, args| {
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
    }
    if(2) => |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?
        } else {
            Atom::Null
        })
    }
    ifelse(3) => |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?
        } else {
            args[2].eval(state)?
        })
    }
    while(2) => |state, args| {
        while args[0].eval(state)?.bool()? {
            args[1].eval(state)?;
        }
        Ok(Atom::Null)
    }
    def(_) => |state, args| {
        if args.len() < 2 {
            return Exception::new_err(
                format!("too few arguments passed to `def`: expected at least 2, found {}", args.len()), 
                Error::Argument
            );
        }
        if let Argument::Variable(var) = &args[0] {
            if let Argument::FunctionCall(inner) = args.last().unwrap() {
                let body = inner.clone();
                let function_arg_names = args[1..args.len() - 1]
                    .iter()
                    .cloned()
                    .map(|fn_arg| match fn_arg {
                        Argument::Variable(fn_arg) => Ok(fn_arg),
                        _ => Exception::new_err(
                            "Error during definition: invalid args were given!",
                            Error::Assign,
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
                Exception::new_err(
                    "Error during definition: no valid function body was given!",
                    Error::Assign,
                )
            }
        } else {
            Exception::new_err(
                "Error during definition: no valid variable was given to define to!",
                Error::Assign,
            )
        }
    }
    import(1) => |state, args| {
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
            if *item.file_name() == *format!("{name}.prog") {
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

        let (atom, imported_state) = prelude::run(&code, &state.file_directory, None);
        let atom = atom?;
        
        for (k, v) in imported_state.storage {
            state.storage.insert(k, v);
        }
        Ok(atom)
    }
    error(1) => |state, args| {
        Exception::new_err(args[0].eval(state)?.string()?, Error::UserRaised)
    }
    catch(1) => |state, args| {
        Ok(args[0]
            .eval(state)
            .unwrap_or_else(|exc| Atom::String(exc.to_string())))
    }
    "=="(2) => |state, args| {
        Ok(Atom::Bool(args[0].eval(state)? == args[1].eval(state)?))
    }
    assert(1) => |state, args| {
        if args[0].eval(state)?.bool()? {
            Ok(Atom::Null)
        } else {
            Exception::new_err("Assertion failed!", Error::Assertion)
        }
    }
    assert_eq(2) => |state, args| {
        let lhs = args[0].eval(state)?;
        let rhs = args[1].eval(state)?;
        if lhs == rhs {
            Ok(Atom::Null)
        } else {
            Exception::new_err(
                format!("Equality assertion failed! lhs: `{lhs}`, rhs: `{rhs}`!"), 
                Error::Assertion
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
