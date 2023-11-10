use crate::{Atom, Function, ProgError};

pub fn functions() -> Vec<Function> {
    vec![
        list(),
        push(),
        index(),
    ]
}

fn list() -> Function {
    Function {
        name: String::from("list"),
        argc: None,
        callback: |functions, storage, args| {
            let mut list = vec![];
            for arg in args {
                list.push(arg.eval(functions, storage)?);
            }
            Ok(Atom::List(list))
        },
    }
}

fn push() -> Function {
    Function {
        name: String::from("push"),
        argc: None,
        callback: |functions, storage, args| {
            args[0]
                .eval(functions, storage)?
                .list()?
                .push(args[1].eval(functions, storage)?);
            Ok(Atom::Null)
        },
    }
}
fn index() -> Function {
    Function {
        name: String::from("list"),
        argc: None,
        callback: |functions, storage, args| {
            Ok(Atom::Int(
                args[0]
                    .eval(functions, storage)?
                    .list()?
                    .get(args[1].eval(functions, storage)?.int()? as usize)
                    .ok_or(ProgError("Unable to index list!".to_string()))?
                    .int()?,
            ))
        },
    }
}
