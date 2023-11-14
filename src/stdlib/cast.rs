use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![int(), string(), bool_fn(), is_null()]
}

fn int() -> Function {
    Function {
        name: String::from("int"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Int(args[0].eval(program, storage)?.int()?))
        }),
    }
}

fn string() -> Function {
    Function {
        name: String::from("string"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::String(args[0].eval(program, storage)?.string()?))
        }),
    }
}
fn bool_fn() -> Function {
    Function {
        name: String::from("bool"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(args[0].eval(program, storage)?.bool()?))
        }),
    }
}

fn is_null() -> Function {
    Function {
        name: String::from("is_null"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(matches!(
                args[0].eval(program, storage)?,
                Atom::Null
            )))
        }),
    }
}
