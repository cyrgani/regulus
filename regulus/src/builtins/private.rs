//! Builtin functions which will never have a stable equivalent and are for internal use only.

use crate::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn epoch_duration() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("internal time error")
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
    /// Constructs an empty list.
    "__builtin_new_list"(0) => |_, _| {
        Ok(Atom::List(vec![]))
    }
}
