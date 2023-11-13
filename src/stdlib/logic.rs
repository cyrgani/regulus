use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![
        or(),
        and(),
        less(),
        less_equals(),
        greater(),
        greater_equals(),
    ]
}

fn or() -> Function {
    Function {
        name: String::from("or"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(
                args[0].eval(program, storage)?.bool()?
                    || args[1].eval(program, storage)?.bool()?,
            ))
        }),
    }
}

fn and() -> Function {
    Function {
        name: String::from("and"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(
                args[0].eval(program, storage)?.bool()?
                    && args[1].eval(program, storage)?.bool()?,
            ))
        }),
    }
}

fn less() -> Function {
    Function {
        name: String::from("<"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(
                args[0].eval(program, storage)?.int()? < args[1].eval(program, storage)?.int()?,
            ))
        }),
    }
}
fn less_equals() -> Function {
    Function {
        name: String::from("<="),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(
                args[0].eval(program, storage)?.int()? <= args[1].eval(program, storage)?.int()?,
            ))
        }),
    }
}

fn greater() -> Function {
    Function {
        name: String::from(">"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(
                args[0].eval(program, storage)?.int()? > args[1].eval(program, storage)?.int()?,
            ))
        }),
    }
}
fn greater_equals() -> Function {
    Function {
        name: String::from(">="),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(
                args[0].eval(program, storage)?.int()? >= args[1].eval(program, storage)?.int()?,
            ))
        }),
    }
}
