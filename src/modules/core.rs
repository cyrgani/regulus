use crate::{Argument, Atom, Function, ProgError};
use std::rc::Rc;

pub fn functions() -> Vec<Function> {
    vec![run(), assign(), if_fn(), ifelse(), while_fn(), def()]
}

fn run() -> Function {
    Function {
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
        name: String::from("assign"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            if let Argument::Variable(var) = &args[0] {
                let val = args[1].eval(program, storage)?;
                storage.insert(var.clone(), val);
                Ok(Atom::Null)
            } else {
                Err(ProgError(
                    "Error during assignment: no variable was given to assign to!".to_string(),
                ))
            }
        }),
    }
}

fn if_fn() -> Function {
    Function {
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
        name: String::from("def"),
        argc: Some(3),
        callback: Rc::new(|program, storage, args| {
            if let Argument::Variable(var) = &args[0] {
                if let Argument::Variable(arg) = &args[1] {
                    if let Argument::FunctionCall(inner) = &args[2] {
                        let body = inner.clone();
                        let arg = arg.clone();
                        let function = Function {
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
                        Err(ProgError(
                            "Error during definition: no function body was given!".to_string(),
                        ))
                    }
                } else {
                    Err(ProgError(
                        "Error during definition: no argument was given!".to_string(),
                    ))
                }
            } else {
                Err(ProgError(
                    "Error during definition: no variable was given to define to!".to_string(),
                ))
            }
        }),
    }
}
