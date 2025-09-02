use crate::exception::TypeError;
use crate::prelude::*;

// TODO: try making as many of these as possible stl functions
functions! {
    // TODO: implement this directly in the STL
    /// Converts the given string into an integer, raising an exception if it is not possible to cast.
    "__builtin_str_to_int"(1) => |state, args| {
        args[0].eval_string(state)?.parse().map(Atom::Int).map_err(|e| state.raise(TypeError, format!("cannot convert string to int: {e}")))
    }
    // TODO: implement this directly in the STL
    /// Converts the given integer into a string.
    "__builtin_int_to_str"(1) => |state, args| {
        Ok(Atom::String(args[0].eval_int(state)?.to_string()))
    }
    // TODO: invent some way for objects to define how they want to be printed.
    /// Evaluates the given arg and returns a string representation of it.
    /// See the documentation of `string(1)` for a comparison of these two methods.
    /// Note that the exact output format is not yet stable and may change, especially regarding
    /// objects.
    ///
    /// This is identical to the output of `write`.
    "printable"(1) => |state, args| {
        Ok(Atom::String(args[0].eval(state)?.to_string()))
    }
}
