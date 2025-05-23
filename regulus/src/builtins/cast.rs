use crate::prelude::*;

fn cast_error_builder(atom: &Atom, new_type: &str) -> Exception {
    Exception::new(format!("Unable to cast {atom} to {new_type}"), Error::Type)
}

functions! {
    /// Converts the given value into an integer, raising an exception if it is not possible to cast.
    /// TODO document the exact conditions and rules
    "int"(1) => |state, args| {
        let atom = args[0].eval(state)?;
        Ok(Atom::Int(match &*atom {
            Atom::Int(val) => *val,
            Atom::Bool(val) => i64::from(*val),
            Atom::String(val) => val
                .parse::<i64>()
                .map_err(|_| cast_error_builder(&atom, "int"))?,
            _ => return Err(cast_error_builder(&atom, "int")),
        }))
    }
    /// Converts the given value into a string, raising an exception if it is not possible to cast.
    /// TODO document the exact conditions and rules
    "string"(1) => |state, args| {
        let atom = args[0].eval(state)?;
        Ok(match *atom {
            Atom::Int(val) => Atom::String(val.to_string()),
            Atom::Bool(val) => Atom::String(val.to_string()),
            Atom::String(_) => atom.into_owned(),
            Atom::Null => Atom::String("null".to_string()),
            _ => return Err(cast_error_builder(&atom, "string")),
        })
    }
    /// Converts the given value into a boolean, raising an exception if it is not possible to cast.
    /// TODO document the exact conditions and rules
    "bool"(1) => |state, args| {
        let atom = args[0].eval(state)?;
        Ok(Atom::Bool(match *atom {
            Atom::Int(val) => val != 0,
            Atom::Bool(val) => val,
            Atom::Null => false,
            _ => return Err(cast_error_builder(&atom, "bool")),
        }))
    }
}
