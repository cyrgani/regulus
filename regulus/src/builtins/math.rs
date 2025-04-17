use crate::prelude::*;

fn arithmetic_operation(
    state: &mut State,
    args: &[Argument],
    name: &str,
    f: fn(i64, i64) -> Option<i64>,
) -> Result<Atom> {
    match f(args[0].eval(state)?.int()?, args[1].eval(state)?.int()?) {
        Some(i) => Ok(Atom::Int(i)),
        None => raise!(Error::Overflow, "overflow occured during {name}!"),
    }
}

functions! {
    /// Adds the two given integers and returns the result, causing an exception in case of overflow.
    "+"(2) => |state, args| arithmetic_operation(state, args, "+", i64::checked_add)
    /// Subtracts the two given integers and returns the result, causing an exception in case of overflow.
    "-"(2) => |state, args| arithmetic_operation(state, args, "-", i64::checked_sub)
    /// Multiplies the two given integers and returns the result, causing an exception in case of overflow.
    "*"(2) => |state, args| arithmetic_operation(state, args, "*", i64::checked_mul)
    /// Divides the two given integers and returns the result, causing an exception in case of division by zero.
    "/"(2) => |state, args| arithmetic_operation(state, args, "/", i64::checked_div)
    /// Calculates the remainder of the two given integers and returns the result, 
    /// causing an exception in case of division by zero.
    "%"(2) => |state, args| arithmetic_operation(state, args, "%", i64::checked_rem)
}
