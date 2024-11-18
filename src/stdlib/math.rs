use crate::prelude::*;
use crate::stdlib::NamedFunction;

pub fn functions() -> Vec<NamedFunction> {
    vec![add(), subtract(), multiply(), divide(), modulo()]
}

fn arithmetic_fn_builder(
    name: &'static str,
    operation_name: &'static str,
    f: fn(i64, i64) -> Option<i64>,
) -> NamedFunction {
    (
        name,
        Function {
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
        },
    )
}

fn add() -> NamedFunction {
    arithmetic_fn_builder("+", "addition", i64::checked_add)
}

fn subtract() -> NamedFunction {
    arithmetic_fn_builder("-", "subtraction", i64::checked_sub)
}

fn multiply() -> NamedFunction {
    arithmetic_fn_builder("*", "multiplication", i64::checked_mul)
}

fn divide() -> NamedFunction {
    arithmetic_fn_builder("/", "division", i64::checked_div)
}

fn modulo() -> NamedFunction {
    arithmetic_fn_builder("%", "modulo", i64::checked_rem)
}
