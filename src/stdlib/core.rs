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
        def_str(),
		error(),
    ]
}

fn run_fn() -> Function {
    Function {
        aliases: vec![],
        name: String::from("_"),
        argc: None,
        callback: Rc::new(|program, storage, args| {
            for arg in args {
                arg.eval(program, storage)?;
            }
            Ok(Atom::Null)
        }),
    }
}

fn assign() -> Function {
    Function {
        aliases: vec![],
        name: String::from("assign"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            if let Argument::Variable(var) = &args[0] {
                let val = args[1].eval(program, storage)?;
                storage.insert(var.clone(), val);
                Ok(Atom::Null)
            } else {
                Err(ProgError {
                    msg: "Error during assignment: no variable was given to assign to!".to_string(),
                    class: AssignError,
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
        callback: Rc::new(|program, storage, args| {
            Ok(if args[0].eval(program, storage)?.bool()? {
                args[1].eval(program, storage)?
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
        callback: Rc::new(|program, storage, args| {
            Ok(if args[0].eval(program, storage)?.bool()? {
                args[1].eval(program, storage)?
            } else {
                args[2].eval(program, storage)?
            })
        }),
    }
}

fn while_fn() -> Function {
    Function {
        aliases: vec![],
        name: String::from("while"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            while args[0].eval(program, storage)?.bool()? {
                args[1].eval(program, storage)?;
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
        callback: Rc::new(|_program, storage, args| {
            println!("Warning: The def-function is broken when combined with importing and the def_str function should be used instead for now.");
            if let Argument::Variable(var) = &args[0] {
                if let Argument::Variable(arg) = &args[1] {
                    if let Argument::FunctionCall(inner) = &args[2] {
                        let body = inner.clone();
                        let arg = arg.clone();
                        let function = Function {
                            aliases: vec![],
                            name: var.clone(),
                            argc: Some(1),
                            callback: Rc::new(move |program, storage, args| {
                                let mut new_storage = storage.clone();
                                new_storage.insert(arg.clone(), args[0].eval(program, storage)?);
                                body.eval(program, &mut new_storage)
                            }),
                        };
                        storage.insert(var.clone(), Atom::Function(function));
                        Ok(Atom::Null)
                    } else {
                        Err(ProgError {
                            msg: "Error during definition: no function body was given!".to_string(),
                            class: AssignError,
                        })
                    }
                } else {
                    Err(ProgError {
                        msg: "Error during definition: no argument was given!".to_string(),
                        class: AssignError,
                    })
                }
            } else {
                Err(ProgError {
                    msg: "Error during definition: no variable was given to define to!".to_string(),
                    class: AssignError,
                })
            }
        }),
    }
}

fn def_str() -> Function {
    Function {
        aliases: vec![],
        name: String::from("def_str"),
        argc: Some(3),
        callback: Rc::new(|_program, storage, args| {
            if let Argument::Variable(var) = &args[0] {
                if let Argument::Variable(arg) = &args[1] {
                    if let Argument::Atom(Atom::String(inner)) = &args[2] {
                        let body = inner.clone();
                        let arg = arg.clone();
                        let function = Function {
                            aliases: vec![],
                            name: var.clone(),
                            argc: Some(1),
                            callback: Rc::new(move |program, storage, args| {
                                let mut new_storage = storage.clone();
                                new_storage.insert(arg.clone(), args[0].eval(program, storage)?);
                                run(&body, Some(new_storage)).map(|(atom, _)| atom)

                                //body.eval(program, &mut new_storage)
                            }),
                        };
                        storage.insert(var.clone(), Atom::Function(function));
                        Ok(Atom::Null)
                    } else {
                        Err(ProgError {
                            msg: "Error during definition: no function body was given!".to_string(),
                            class: AssignError,
                        })
                    }
                } else {
                    Err(ProgError {
                        msg: "Error during definition: no argument was given!".to_string(),
                        class: AssignError,
                    })
                }
            } else {
                Err(ProgError {
                    msg: "Error during definition: no variable was given to define to!".to_string(),
                    class: AssignError,
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
        callback: Rc::new(|program, storage, args| {
            let path = args[0].eval(program, storage)?.string()?;
            let code = fs::read_to_string(path).map_err(|error| ProgError {
                msg: format!("{}", error),
                class: ImportError,
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
        callback: Rc::new(|program, storage, args| {
            Err(ProgError {
                msg: args[0].eval(program, storage)?.string()?,
                class: UserRaisedError,
            })
        }),
    }
}
