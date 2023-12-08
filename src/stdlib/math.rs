use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![add(), subtract(), multiply()]
}

fn arithmetic_fn_builder(
    name: &str,
    operation_name: &'static str,
    f: fn(i32, i32) -> Option<i32>,
) -> Function {
    Function {
        aliases: vec![],
        name: String::from(name),
        argc: Some(2),
        callback: Rc::new(move |storage, args| {
            match f(
                args[0].eval(storage)?.int()?,
                args[1].eval(storage)?.int()?,
            ) {
                Some(i) => Ok(Atom::Int(i)),
                None => Err(Exception {
                    msg: format!("overflow occured during {}!", operation_name),
                    error: Error::Overflow,
                }),
            }
        }),
    }
}

fn add() -> Function {
    arithmetic_fn_builder("+", "addition", |lhs, rhs| lhs.checked_add(rhs))
}

fn subtract() -> Function {
    arithmetic_fn_builder("-", "subtraction", |lhs, rhs| lhs.checked_sub(rhs))
}

fn multiply() -> Function {
    arithmetic_fn_builder("*", "multiplication", |lhs, rhs| lhs.checked_mul(rhs))
}
