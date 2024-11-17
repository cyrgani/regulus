use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![add(), subtract(), multiply()]
}

fn arithmetic_fn_builder(
    name: &str,
    operation_name: &'static str,
    f: fn(i64, i64) -> Option<i64>,
) -> Function {
    Function {
        aliases: vec![],
        name: String::from(name),
        argc: Some(2),
        callback: Rc::new(move |state, args| {
            match f(args[0].eval(state)?.int()?, args[1].eval(state)?.int()?) {
                Some(i) => Ok(Atom::Int(i)),
                None => Exception::new_err(
                    format!("overflow occured during {operation_name}!"),
                    Error::Overflow,
                ),
            }
        }),
    }
}

fn add() -> Function {
    arithmetic_fn_builder("+", "addition", i64::checked_add)
}

fn subtract() -> Function {
    arithmetic_fn_builder("-", "subtraction", i64::checked_sub)
}

fn multiply() -> Function {
    arithmetic_fn_builder("*", "multiplication", i64::checked_mul)
}
