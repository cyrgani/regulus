use crate::exception::TypeError;
use crate::prelude::*;

fn cast_error_builder(atom: &Atom, new_type: &str, state: &State) -> Exception {
    state.raise(TypeError, format!("Unable to cast {atom} to {new_type}"))
}

// TODO: try making as many of these as possible stl functions
functions! {
    // TODO: implement this directly in the STL
    /// Converts the given string into an integer, raising an exception if it is not possible to cast.
    "__builtin_str_to_int"(1) => |state, args| {
        args[0].eval_string(state)?.parse().map(Atom::Int).map_err(|e| state.raise(TypeError, format!("cannot convert string to int: {e}")))
    }
    /// Converts the given value into a string, raising an exception if it is not possible to cast.
    ///
    /// This method is fallible and is currently only able to cast ints, bools, strings and nulls (to "null").
    /// If you want to display an arbitrary atom (such as for error messages), use `printable(1)`
    /// instead, which is infallible.
    "string"(1) => |state, args| {
        let atom = args[0].eval(state)?.into_owned();
        Ok(match atom {
            Atom::Int(val) => Atom::String(val.to_string()),
            Atom::Bool(val) => Atom::String(val.to_string()),
            Atom::String(_) => atom,
            Atom::Null => Atom::String("null".to_string()),
            _ => return Err(cast_error_builder(&atom, "string", state)),
        })
    }
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
