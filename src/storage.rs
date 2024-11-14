use crate::prelude::*;
use std::collections::HashMap;

pub type Storage = HashMap<String, Atom>;

pub fn initial() -> Storage {
    crate::function::all_functions()
        .into_iter()
        .map(|f| (f.name.clone(), Atom::Function(f)))
        .collect()
}

pub fn get_function(name: &str, storage: &Storage) -> ProgResult<Function> {
    storage
        .values()
        .find_map(|atom| match atom {
            Atom::Function(function) if function.name == name => Some(function.clone()),
            _ => None,
        })
        .ok_or_else(|| Exception::new(format!("No function `{name}` found!"), Error::Name))
}
