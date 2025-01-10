use crate::prelude::*;

export! {
    int, string, bool_fn,
}

fn cast_error_builder(atom: &Atom, new_type: &str) -> Exception {
    Exception::new(format!("Unable to cast {atom} to {new_type}"), Error::Type)
}

function! {
    name: int,
    argc: Some(1),
    callback: |state, args| {
        let atom = args[0].eval(state)?;
        Ok(Atom::Int(match &atom {
            Atom::Int(val) => *val,
            Atom::Bool(val) => i64::from(*val),
            Atom::String(val) => val
                .parse::<i64>()
                .map_err(|_error| cast_error_builder(&atom, "int"))?,
            _ => return Err(cast_error_builder(&atom, "int")),
        }))
    },
}

function! {
    name: string,
    argc: Some(1),
    callback: |state, args| {
        let atom = args[0].eval(state)?;
        Ok(Atom::String(match &atom {
            Atom::Int(val) => val.to_string(),
            Atom::Bool(val) => val.to_string(),
            Atom::String(val) => val.clone(),
            Atom::Null => "null".to_string(),
            _ => return Err(cast_error_builder(&atom, "string")),
        }))
    },
}

function! {
    name: bool_fn,
    override_name: bool,
    argc: Some(1),
    callback: |state, args| {
        let atom = args[0].eval(state)?;
        Ok(Atom::Bool(match &atom {
            Atom::Int(val) => *val != 0,
            Atom::Bool(val) => *val,
            Atom::Null => false,
            _ => return Err(cast_error_builder(&atom, "bool")),
        }))
    },
}
