use crate::prelude::*;

fn arithmetic_operation(
    state: &mut State,
    args: &[Argument],
    name: &str,
    f: fn(i64, i64) -> Option<i64>,
) -> Result<Atom> {
    let lhs = args[0].eval(state)?.int()?;
    let rhs = args[1].eval(state)?.int()?;

    if let Some(i) = f(lhs, rhs) {
        Ok(Atom::Int(i))
    } else {
        if name == "/" && rhs == 0 {
            raise!(Error::DivideByZero, "attempted to divide by zero")
        }
        raise!(Error::Overflow, "overflow occured during {name}")
    }
}

fn shift_operation(
    state: &mut State,
    args: &[Argument],
    name: &str,
    f: fn(i64, u32) -> Option<i64>,
) -> Result<Atom> {
    let lhs = args[0].eval(state)?.int()?;
    let rhs = u32::try_from(args[1].eval(state)?.int()?).map_err(|err| {
        state.raise(
            Error::Argument,
            format!("invalid arithmetic argument for `{name}`: `{err}`"),
        )
    })?;
    if let Some(i) = f(lhs, rhs) {
        Ok(Atom::Int(i))
    } else {
        raise!(Error::Overflow, "{name} operation failed")
    }
}

functions! {
    /// Adds the two given integers and returns the result, causing an exception in case of overflow.
    "__builtin_int_add"(2) => |state, args| arithmetic_operation(state, args, "+", i64::checked_add)
    /// Subtracts the two given integers and returns the result, causing an exception in case of overflow.
    "__builtin_int_sub"(2) => |state, args| arithmetic_operation(state, args, "-", i64::checked_sub)
    /// Multiplies the two given integers and returns the result, causing an exception in case of overflow.
    "__builtin_int_mul"(2) => |state, args| arithmetic_operation(state, args, "*", i64::checked_mul)
    /// Divides the two given integers and returns the result, causing an exception in case of division by zero.
    "__builtin_int_div"(2) => |state, args| arithmetic_operation(state, args, "/", i64::checked_div)
    /// Calculates the remainder of the two given integers and returns the result,
    /// causing an exception in case of division by zero.
    "%"(2) => |state, args| arithmetic_operation(state, args, "%", i64::checked_rem)
    /// Shifts the first integer to the left by the second amount of digits,
    /// causing an exception in case of overflow or a negative shift amount.
    "<<"(2) => |state, args| shift_operation(state, args, "<<", i64::checked_shl)
    /// Shifts the first integer to the right by the second amount of digits,
    /// causing an exception in case of overflow or a negative shift amount.
    ">>"(2) => |state, args| shift_operation(state, args, ">>", i64::checked_shr)
}
