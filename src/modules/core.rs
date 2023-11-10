use crate::{Argument, Atom, Function, ProgError};

pub fn functions() -> Vec<Function> {
    vec![run(), assign(), if_fn(), ifelse(), while_fn()]
}

fn run() -> Function {
    Function {
        name: String::from("_"),
        argc: None,
        callback: |functions, storage, args| {
            for arg in args {
                arg.eval(functions, storage)?;
            }
            Ok(Atom::Null)
        },
    }
}

fn assign() -> Function {
    Function {
        name: String::from("assign"),
        argc: Some(2),
        callback: |functions, storage, args| {
            if let Argument::Variable(var) = &args[0] {
                let val = args[1].eval(functions, storage)?;
                storage.insert(var.clone(), val);
                Ok(Atom::Null)
            } else {
                Err(ProgError(
                    "Error during assignment: no variable was given to assign to!".to_string(),
                ))
            }
        },
    }
}

fn if_fn() -> Function {
    Function {
        name: String::from("if"),
        argc: Some(2),
        callback: |functions, storage, args| {
            Ok(if args[0].eval(functions, storage)?.bool()? {
				args[1].eval(functions, storage)?
			} else {
				Atom::Null
			})
        },
    }
}

fn ifelse() -> Function {
    Function {
        name: String::from("ifelse"),
        argc: Some(3),
        callback: |functions, storage, args| {
            Ok(if args[0].eval(functions, storage)?.bool()? {
				args[1].eval(functions, storage)?
			} else {
				args[2].eval(functions, storage)?
			})
        },
    }
}

fn while_fn() -> Function {
    Function {
        name: String::from("while"),
        argc: Some(2),
        callback: |functions, storage, args| {
            while args[0].eval(functions, storage)?.bool()? {
				args[1].eval(functions, storage)?;
			}
			Ok(Atom::Null)
        },
    }
}