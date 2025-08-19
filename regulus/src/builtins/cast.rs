use crate::exception::TypeError;
use crate::prelude::*;

fn cast_error_builder(atom: &Atom, new_type: &str, state: &State) -> Exception {
    state.raise(TypeError, format!("Unable to cast {atom} to {new_type}"))
}

functions! {
    /// Converts the given value into an integer, raising an exception if it is not possible to cast.
    /// TODO document the exact conditions and rules
    "int"(1) => |state, args| {
        let atom = args[0].eval(state)?.into_owned();
        Ok(Atom::Int(match atom {
            Atom::Int(val) => val,
            Atom::Bool(val) => i64::from(val),
            Atom::String(ref val) => val
                .parse::<i64>()
                .map_err(|_| cast_error_builder(&atom, "int", state))?,
            _ => return Err(cast_error_builder(&atom, "int", state)),
        }))
    }
    /// Converts the given value into a string, raising an exception if it is not possible to cast.
    /// TODO document the exact conditions and rules
    /// 
    /// This method is fallible and is currently only able to cast ints, bools, strings and nulls.
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
    /// Converts the given value into a boolean, raising an exception if it is not possible to cast.
    /// TODO document the exact conditions and rules
    "bool"(1) => |state, args| {
        let atom = args[0].eval(state)?.into_owned();
        Ok(Atom::Bool(match atom {
            Atom::Int(val) => val != 0,
            Atom::Bool(val) => val,
            Atom::Null => false,
            _ => return Err(cast_error_builder(&atom, "bool", state)),
        }))
    }
    /// Evaluates the given arg and returns a string representation of it.
    /// See the documentation of `string(1)` for a comparison of these two methods.
    /// Note that the exact output format is not yet stable and may change, especially regarding 
    /// objects. 
    "printable"(1) => |state, args| {
        Ok(Atom::String(args[0].eval(state)?.to_string()))
    }
}
