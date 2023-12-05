use crate::prelude::*;
use std::collections::HashMap;

pub type Storage = HashMap<String, Atom>;

pub fn initial_storage() -> Storage {
    crate::function::all_functions()
        .into_iter()
        .map(|f| (f.name.clone(), Atom::Function(f)))
        .collect()
}

pub fn get_function(name: &str, storage: &Storage) -> ProgResult<Function> {
    storage
        .values()
        .find_map(|atom| {
            if let Atom::Function(function) = atom {
                if function.name == name {
                    Some(function.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .ok_or(Exception {
            msg: format!("No function `{name}` found!"),
            error: Error::Name,
        })
}
