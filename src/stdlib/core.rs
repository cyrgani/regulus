use crate::prelude::*;
use std::fs;

pub fn functions() -> Vec<Function> {
    vec![
        run_fn(),
        assign(),
        if_fn(),
        ifelse(),
        while_fn(),
        def(),
        import(),
        error(),
        equals(),
        assert(),
        catch(),
    ]
}

fn run_fn() -> Function {
    Function {
        aliases: vec!["run".to_string()],
        name: String::from("_"),
        argc: None,
        callback: Rc::new(|storage, args| {
            for arg in args {
                arg.eval(storage)?;
            }
            Ok(Atom::Null)
        }),
    }
}

fn assign() -> Function {
    Function {
        aliases: vec!["=".to_string()],
        name: String::from("assign"),
        argc: Some(2),
        callback: Rc::new(|storage, args| {
            if let Argument::Variable(var) = &args[0] {
                let val = args[1].eval(storage)?;
                storage.insert(var.clone(), val);
                Ok(Atom::Null)
            } else {
                Err(Exception {
                    msg: "Error during assignment: no variable was given to assign to!".to_string(),
                    error: Error::Assign,
                })
            }
        }),
    }
}

fn if_fn() -> Function {
    Function {
        aliases: vec![],
        name: String::from("if"),
        argc: Some(2),
        callback: Rc::new(|storage, args| {
            Ok(if args[0].eval(storage)?.bool()? {
                args[1].eval(storage)?
            } else {
                Atom::Null
            })
        }),
    }
}

fn ifelse() -> Function {
    Function {
        aliases: vec![],
        name: String::from("ifelse"),
        argc: Some(3),
        callback: Rc::new(|storage, args| {
            Ok(if args[0].eval(storage)?.bool()? {
                args[1].eval(storage)?
            } else {
                args[2].eval(storage)?
            })
        }),
    }
}

fn while_fn() -> Function {
    Function {
        aliases: vec![],
        name: String::from("while"),
        argc: Some(2),
        callback: Rc::new(|storage, args| {
            while args[0].eval(storage)?.bool()? {
                args[1].eval(storage)?;
            }
            Ok(Atom::Null)
        }),
    }
}

fn def() -> Function {
    Function {
        aliases: vec![],
        name: String::from("def"),
        argc: Some(3),
        callback: Rc::new(|storage, args| {
            if let Argument::Variable(var) = &args[0] {
                if let Argument::Variable(arg) = &args[1] {
                    if let Argument::FunctionCall(inner) = &args[2] {
                        let body = inner.clone();
                        let arg = arg.clone();
                        let function = Function {
                            aliases: vec![],
                            name: var.clone(),
                            argc: Some(1),
                            callback: Rc::new(move |storage, args| {
                                let mut new_storage = storage.clone();
                                new_storage.insert(arg.clone(), args[0].eval(storage)?);
                                body.eval(&mut new_storage)
                            }),
                        };
                        storage.insert(var.clone(), Atom::Function(function));
                        Ok(Atom::Null)
                    } else {
                        Err(Exception {
                            msg: "Error during definition: no function body was given!".to_string(),
                            error: Error::Assign,
                        })
                    }
                } else {
                    Err(Exception {
                        msg: "Error during definition: no argument was given!".to_string(),
                        error: Error::Assign,
                    })
                }
            } else {
                Err(Exception {
                    msg: "Error during definition: no variable was given to define to!".to_string(),
                    error: Error::Assign,
                })
            }
        }),
    }
}

fn import() -> Function {
    Function {
        aliases: vec![],
        name: String::from("import"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            let path = args[0].eval(storage)?.string()?;
            let code = fs::read_to_string(path).map_err(|error| Exception {
                msg: format!("{}", error),
                error: Error::Import,
            })?;
            let (atom, imported_storage) = run(&code, None)?;

            for (k, v) in imported_storage {
                storage.insert(k, v);
            }
            Ok(atom)
        }),
    }
}

fn error() -> Function {
    Function {
        aliases: vec![],
        name: String::from("error"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            Err(Exception {
                msg: args[0].eval(storage)?.string()?,
                error: Error::UserRaised,
            })
        }),
    }
}

fn catch() -> Function {
    Function {
        aliases: vec![],
        name: String::from("catch"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            Ok(args[0]
                .eval(storage)
                .unwrap_or_else(|exc| Atom::String(exc.to_string())))
        }),
    }
}

fn equals() -> Function {
    Function {
        aliases: vec!["equals".to_string()],
        name: String::from("=="),
        argc: Some(2),
        callback: Rc::new(|storage, args| {
            Ok(Atom::Bool(args[0].eval(storage)? == args[1].eval(storage)?))
        }),
    }
}

fn assert() -> Function {
    Function {
        aliases: vec![],
        name: String::from("assert"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            if args[0].eval(storage)?.bool()? {
                Ok(Atom::Null)
            } else {
                Err(Exception {
                    msg: "Assertion failed!".to_string(),
                    error: Error::Assertion,
                })
            }
        }),
    }
}
