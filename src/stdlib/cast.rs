use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![int(), string(), bool_fn()]
}

fn cast_error_builder(atom: &Atom, new_type: &str) -> Exception {
    Exception {
        msg: format!("Unable to cast {:?} to {}", atom, new_type),
        error: Error::Type,
    }
}

fn int() -> Function {
    Function {
        aliases: vec![],
        name: String::from("int"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            let atom = args[0].eval(storage)?;
            Ok(Atom::Int(match &atom {
                Atom::Int(val) => *val,
                Atom::Bool(val) => *val as i32,
                Atom::String(val) => val
                    .parse::<i32>()
                    .map_err(|_error| cast_error_builder(&atom, "int"))?,
                _ => return Err(cast_error_builder(&atom, "int")),
            }))
        }),
    }
}

fn string() -> Function {
    Function {
        aliases: vec![],
        name: String::from("string"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            let atom = args[0].eval(storage)?;
            Ok(Atom::String(match &atom {
                Atom::Int(val) => val.to_string(),
                Atom::Bool(val) => val.to_string(),
                Atom::String(val) => val.clone(),
                Atom::Null => "null".to_string(),
                _ => return Err(cast_error_builder(&atom, "string")),
            }))
        }),
    }
}

fn bool_fn() -> Function {
    Function {
        aliases: vec![],
        name: String::from("bool"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            let atom = args[0].eval(storage)?;
            Ok(Atom::Bool(match &atom {
                Atom::Int(val) => *val != 0,
                Atom::Bool(val) => *val,
                Atom::Null => false,
                _ => return Err(cast_error_builder(&atom, "bool")),
            }))
        }),
    }
}
