use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![int(), string(), bool_fn(), is_null()]
}

fn _cast_error(atom: &Atom, new_type: &str) -> ProgError {
    ProgError {
        msg: format!("Unable to cast {:?} to {}", atom, new_type),
        class: TypeError,
    }
}

fn int() -> Function {
    Function {
        aliases: vec![],
        name: String::from("int"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            let atom = args[0].eval(program, storage)?;
            Ok(Atom::Int(match &atom {
                Atom::Int(val) => *val,
                Atom::Bool(val) => *val as i32,
                Atom::String(val) => val
                    .parse::<i32>()
                    .map_err(|_error| _cast_error(&atom, "int"))?,
                _ => return Err(_cast_error(&atom, "int")),
            }))
        }),
    }
}

fn string() -> Function {
    Function {
        aliases: vec![],
        name: String::from("string"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            let atom = args[0].eval(program, storage)?;
            Ok(Atom::String(match &atom {
                Atom::Int(val) => val.to_string(),
                Atom::Bool(val) => val.to_string(),
                Atom::String(val) => val.clone(),
                Atom::Null => "null".to_string(),
                _ => return Err(_cast_error(&atom, "string")),
            }))
        }),
    }
}

fn bool_fn() -> Function {
    Function {
        aliases: vec![],
        name: String::from("bool"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            let atom = args[0].eval(program, storage)?;
            Ok(Atom::Bool(match &atom {
                Atom::Int(val) => *val != 0,
                Atom::Bool(val) => *val,
                Atom::Null => false,
                _ => return Err(_cast_error(&atom, "bool")),
            }))
        }),
    }
}

fn is_null() -> Function {
    Function {
        aliases: vec![],
        name: String::from("is_null"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            Ok(Atom::Bool(matches!(
                args[0].eval(program, storage)?,
                Atom::Null
            )))
        }),
    }
}
