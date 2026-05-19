//! Builtin functions which are for internal use only.

use crate::prelude::*;
use std::cmp::Ordering;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn epoch_duration() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("internal time error")
}

fn builtin_atom_cmp(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let mode = args[0].eval_mode(state);
    let lhs = args[1].eval(state)?;
    let rhs = args[2].eval(state)?;
    if let Some(o) = lhs.partial_cmp(&rhs) {
        Ok(Atom::Bool(match mode {
            0 => o == Ordering::Less,
            1 => o == Ordering::Less || o == Ordering::Equal,
            2 => o == Ordering::Greater || o == Ordering::Equal,
            3 => o == Ordering::Greater,
            _ => unreachable!(),
        }))
    } else {
        raise!(state, "Argument", "cannot compare {lhs} and {rhs}")
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
        Ok(Atom::Bool(args[0].eval(state)? == args[1].eval(state)?))
    }
    /// Compares both arguments.
    /// Arguments: mode, lhs, rhs.
    /// Raises an exception if the comparison is not supported.
    "__builtin_atom_cmp"(3) => builtin_atom_cmp
    /// Hack since string escape codes do not exist yet.
    "__builtin_cr"(0) => |_, _| Ok(Atom::Char('\r'))
}
