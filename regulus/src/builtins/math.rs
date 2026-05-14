use crate::prelude::*;

fn shift_operation(
    state: &mut State,
    args: &[Argument],
    name: &str,
    f: fn(i64, u32) -> Option<i64>,
) -> Result<Atom> {
    let lhs = args[0].eval_int(state)?;
    let rhs = u32::try_from(args[1].eval_int(state)?).map_err(|err| {
        state.raise(
            "Argument",
            format!("shift amount too big for `{name}`: `{err}`"),
        )
    })?;
    if let Some(i) = f(lhs, rhs) {
        Ok(Atom::Int(i))
    } else {
        raise!(state, "Overflow", "{name} operation failed")
    }
}

fn builtin_int_math(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let mode = args[0].eval_int(state).expect("first arg must be an int");
    let lhs = args[1].eval_int(state)?;
    let rhs = args[2].eval_int(state)?;

    let (name, op): (&str, fn(i64, i64) -> Option<i64>) = match mode {
        0 => ("+", i64::checked_add),
        1 => ("-", i64::checked_sub),
        2 => ("*", i64::checked_mul),
        3 => ("/", i64::checked_div),
        4 => ("%", i64::checked_rem),
        _ => unreachable!(),
    };

    if let Some(i) = op(lhs, rhs) {
        Ok(Atom::Int(i))
    } else {
        if (name == "/" || name == "%") && rhs == 0 {
            raise!(state, "DivideByZero", "attempted to divide by zero")
        }
        raise!(state, "Overflow", "overflow occured during {name}")
    }
}

// TODO: move most of these to the STL
functions! {
    /// Shifts the first integer to the left by the second amount of digits,
    /// causing an exception in case of overflow or a negative shift amount.
    "<<"(2) => |state, args| shift_operation(state, args, "<<", i64::checked_shl)
    /// Shifts the first integer to the right by the second amount of digits,
    /// causing an exception in case of overflow or a negative shift amount.
    ">>"(2) => |state, args| shift_operation(state, args, ">>", i64::checked_shr)
    /// Internal function for integer math.
    "__builtin_int_math"(3) => builtin_int_math
    /// Calculates the XOR of the two given integers and returns the result.
    "__builtin_int_xor"(2) => |state, args| Ok(Atom::Int(args[0].eval_int(state)? ^ args[1].eval_int(state)?))
}
