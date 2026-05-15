use crate::prelude::*;

fn builtin_int_shift(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let mode = args[0].eval_mode(state);
    let (name, op): (&str, fn(i64, u32) -> Option<i64>) = match mode {
        0 => ("<<", i64::checked_shl),
        1 => (">>", i64::checked_shr),
        _ => unreachable!(),
    };

    let lhs = args[1].eval_int(state)?;
    let rhs = u32::try_from(args[2].eval_int(state)?).map_err(|err| {
        state.raise(
            "Argument",
            format!("shift amount too big for `{name}`: `{err}`"),
        )
    })?;
    if let Some(i) = op(lhs, rhs) {
        Ok(Atom::Int(i))
    } else {
        raise!(state, "Overflow", "{name} operation failed")
    }
}

fn builtin_int_math(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let mode = args[0].eval_mode(state);
    let lhs = args[1].eval_int(state)?;
    let rhs = args[2].eval_int(state)?;

    let (name, op): (&str, fn(i64, i64) -> Option<i64>) = match mode {
        0 => ("+", i64::checked_add),
        1 => ("-", i64::checked_sub),
        2 => ("*", i64::checked_mul),
        3 => ("/", i64::checked_div),
        4 => ("%", i64::checked_rem),
        5 => ("^", |l, r| Some(l ^ r)),
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

functions! {
    /// Internal function for integer math.
    "__builtin_int_math"(3) => builtin_int_math
    /// Internal function for integer bit shifts.
    "__builtin_int_shift"(3) => builtin_int_shift
}
