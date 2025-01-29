use crate::prelude::*;

fn arithmetic_operation(
    state: &mut State,
    args: &[Argument],
    name: &str,
    f: fn(i64, i64) -> Option<i64>,
) -> Result<Atom> {
    match f(args[0].eval(state)?.int()?, args[1].eval(state)?.int()?) {
        Some(i) => Ok(Atom::Int(i)),
        None => Exception::new_err(format!("overflow occured during {name}!"), Error::Overflow),
    }
}

functions! {
    // TODO: all after the first require quotes for some reason
    "+"(2) => |state, args| arithmetic_operation(state, args, "+", i64::checked_add)
    "-"(2) => |state, args| arithmetic_operation(state, args, "-", i64::checked_sub)
    "*"(2) => |state, args| arithmetic_operation(state, args, "*", i64::checked_mul)
    "/"(2) => |state, args| arithmetic_operation(state, args, "/", i64::checked_div)
    "%"(2) => |state, args| arithmetic_operation(state, args, "%", i64::checked_rem)
}
