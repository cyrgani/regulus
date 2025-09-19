//! Builtin functions which will never have a stable equivalent and are for internal use only.

use crate::exception::ArgumentError;
use crate::exception::DivideByZeroError;
use crate::exception::OverflowError;
use crate::prelude::*;
use std::cmp::Ordering;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn epoch_duration() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("internal time error")
}

pub(crate) fn arithmetic_operation(
    state: &mut State,
    args: &[Argument],
    name: &str,
    f: fn(i64, i64) -> Option<i64>,
) -> Result<Atom> {
    let lhs = args[0].eval_int(state)?;
    let rhs = args[1].eval_int(state)?;

    if let Some(i) = f(lhs, rhs) {
        Ok(Atom::Int(i))
    } else {
        if name == "/" && rhs == 0 {
            raise!(state, DivideByZeroError, "attempted to divide by zero")
        }
        raise!(state, OverflowError, "overflow occured during {name}")
    }
}

functions! {
    /// Evaluates the given argument, extracts the exception and prints it to stderr.
    /// Not meant to be used outside of tests.
    "__builtin_print_catch"(1) => |state, args| {
        let exc = args[0].eval(state).expect_err("`__builtin_print_catch` arg should cause exception");
        state.write_to_stderr(&exc.to_string());
        state.write_to_stderr("\n");
        Ok(Atom::Null)
    }
    /// Returns the current time in seconds (Unix epoch) as an integer.
    ///
    /// The stable version of this function is in the `time` STL module.
    "__builtin_now"(0) => |state, _| {
        Atom::int_from_rust_int(epoch_duration().as_secs(), state)
    }
    /// Returns the nanosecond part of the current time as an integer.
    ///
    /// The stable version of this function is in the `time` STL module.
    "__builtin_now_nanos_part"(0) => |state, _| {
        Atom::int_from_rust_int(epoch_duration().subsec_nanos(), state)
    }
    /// Evaluates both arguments and returns whether they are equal.
    "__builtin_atom_eq"(2) => |state, args| {
        Ok(Atom::Bool(args[0].eval(state)?.into_owned() == *args[1].eval(state)?))
    }
    /// Compares both arguments.
    /// Returns:
    /// * 0 if they are equal
    /// * 1 if lhs > rhs
    /// * 2 if lhs < rhs
    /// Raises an exception if the comparison is not supported.
    "__builtin_atom_cmp"(2) => |state, args| {
        let lhs = args[0].eval(state)?.into_owned();
        let rhs = args[1].eval(state)?;
        Ok(Atom::Int(match lhs.partial_cmp(rhs.as_ref()) {
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => 1,
            Some(Ordering::Less) => 2,
            None => raise!(state, ArgumentError, "cannot compare {lhs} and {rhs}"),
        }))
    }
    /// Constructs an empty list.
    "__builtin_new_list"(0) => |_, _| {
        Ok(Atom::List(vec![]))
    }
    /// Adds the two given integers and returns the result, causing an exception in case of overflow.
    "__builtin_int_add"(2) => |state, args| arithmetic_operation(state, args, "+", i64::checked_add)
    /// Concatenates the two given strings and returns the result.
    "__builtin_str_add"(2) => |state, args| {
        let mut s = args[0].eval_string(state)?;
        s.push_str(args[1].eval_string(state)?.as_str());
        Ok(Atom::String(s))
    }
    /// Subtracts the two given integers and returns the result, causing an exception in case of overflow.
    "__builtin_int_sub"(2) => |state, args| arithmetic_operation(state, args, "-", i64::checked_sub)
    /// Multiplies the two given integers and returns the result, causing an exception in case of overflow.
    "__builtin_int_mul"(2) => |state, args| arithmetic_operation(state, args, "*", i64::checked_mul)
    /// Divides the two given integers and returns the result, causing an exception in case of division by zero.
    "__builtin_int_div"(2) => |state, args| arithmetic_operation(state, args, "/", i64::checked_div)
}
