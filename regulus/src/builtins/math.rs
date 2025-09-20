use crate::exception::{ArgumentError, OverflowError};
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
            ArgumentError,
            format!("invalid arithmetic argument for `{name}`: `{err}`"),
        )
    })?;
    if let Some(i) = f(lhs, rhs) {
        Ok(Atom::Int(i))
    } else {
        raise!(state, OverflowError, "{name} operation failed")
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
    /// Evaluates both arguments as booleans and performs short-circuiting OR on them.
    "||"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval_bool(state)? ||
        args[1].eval_bool(state)?
    ))
    /// Evaluates both arguments as booleans and performs short-circuiting AND on them.
    "&&"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval_bool(state)? &&
        args[1].eval_bool(state)?
    ))
    /// Evaluates both arguments as integers and preforms XOR.
    "^"(2) => |state, args| Ok(Atom::Int(args[0].eval_int(state)? ^ args[1].eval_int(state)?))
}
