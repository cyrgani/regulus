use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![list(), push(), index()]
}

fn list() -> Function {
    Function {
        name: String::from("list"),
        argc: None,
        callback: Rc::new(|program, storage, args| {
            let mut list = vec![];
            for arg in args {
                list.push(arg.eval(program, storage)?);
            }
            Ok(Atom::List(list))
        }),
    }
}

fn push() -> Function {
    Function {
        name: String::from("push"),
        argc: None,
        callback: Rc::new(|program, storage, args| {
            args[0]
                .eval(program, storage)?
                .list()?
                .push(args[1].eval(program, storage)?);
            Ok(Atom::Null)
        }),
    }
}
fn index() -> Function {
    Function {
        name: String::from("index"),
        argc: None,
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Int(
                args[0]
                    .eval(program, storage)?
                    .list()?
                    .get(args[1].eval(program, storage)?.int()? as usize)
                    .ok_or(ProgError {
                        msg: "Unable to index list!".to_string(),
                        class: crate::ErrorClass::OtherError,
                    })?
                    .int()?,
            ))
        }),
    }
}
