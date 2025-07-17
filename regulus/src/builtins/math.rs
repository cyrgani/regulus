use crate::prelude::*;

fn arithmetic_operation<T: TryFrom<i64, Error: std::fmt::Display>>(
    state: &mut State,
    args: &[Argument],
    name: &str,
    f: fn(i64, T) -> Option<i64>,
) -> Result<Atom> {
    match f(
        args[0].eval(state)?.int()?,
        T::try_from(args[1].eval(state)?.int()?).map_err(|err| {
            Exception::new(
                format!("invalid arithmetic argument for `{name}`: `{err}`"),
                Error::Argument,
            )
        })?,
    ) {
        Some(i) => Ok(Atom::Int(i)),
        // TODO: this error could also be division by zero
        None => raise!(Error::Overflow, "overflow occured during {name}!"),
    }
}

functions! {
    /// Adds the two given integers and returns the result, causing an exception in case of overflow.
    "__builtin_int_add"(2) => |state, args| arithmetic_operation(state, args, "+", i64::checked_add)
    /// Subtracts the two given integers and returns the result, causing an exception in case of overflow.
    "-"(2) => |state, args| arithmetic_operation(state, args, "-", i64::checked_sub)
    /// Multiplies the two given integers and returns the result, causing an exception in case of overflow.
    "*"(2) => |state, args| arithmetic_operation(state, args, "*", i64::checked_mul)
    /// Divides the two given integers and returns the result, causing an exception in case of division by zero.
    "/"(2) => |state, args| arithmetic_operation(state, args, "/", i64::checked_div)
    /// Calculates the remainder of the two given integers and returns the result,
    /// causing an exception in case of division by zero.
    "%"(2) => |state, args| arithmetic_operation(state, args, "%", i64::checked_rem)
    /// Shifts the first integer to the left by the second amount of digits,
    /// causing an exception in case of overflow or a negative shift amount.
    "<<"(2) => |state, args| arithmetic_operation(state, args, "<<", i64::checked_shl)
    /// Shifts the first integer to the right by the second amount of digits,
    /// causing an exception in case of overflow or a negative shift amount.
    ">>"(2) => |state, args| arithmetic_operation(state, args, ">>", i64::checked_shr)
}
