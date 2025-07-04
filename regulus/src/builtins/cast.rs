use crate::parsing::positions::Span;
use crate::prelude::*;

fn cast_error_builder(atom: &Atom, new_type: &str, current_span: Span) -> Exception {
    Exception::spanned(format!("Unable to cast {atom} to {new_type}"), Error::Type, current_span)
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
                .map_err(|_| cast_error_builder(&atom, "int", state.current_span))?,
            _ => return Err(cast_error_builder(&atom, "int", state.current_span)),
        }))
    }
    /// Converts the given value into a string, raising an exception if it is not possible to cast.
    /// TODO document the exact conditions and rules
    "string"(1) => |state, args| {
        let atom = args[0].eval(state)?.into_owned();
        Ok(match atom {
            Atom::Int(val) => Atom::String(val.to_string()),
            Atom::Bool(val) => Atom::String(val.to_string()),
            Atom::String(_) => atom,
            Atom::Null => Atom::String("null".to_string()),
            _ => return Err(cast_error_builder(&atom, "string", state.current_span)),
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
            _ => return Err(cast_error_builder(&atom, "bool", state.current_span)),
        }))
    }
}
