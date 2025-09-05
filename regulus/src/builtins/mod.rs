use crate::prelude::Atom;
use std::collections::HashMap;

mod core;
mod help;
mod io;
mod list;
mod logic;
mod math;
mod private;
mod string;
mod ty;

pub fn all_functions() -> HashMap<String, Atom> {
    let mut functions = HashMap::new();

    for module in [
        core::functions(),
        help::functions(),
        io::functions(),
        list::functions(),
        logic::functions(),
        math::functions(),
        private::functions(),
        string::functions(),
        ty::functions(),
    ] {
        for (name, function) in module {
            functions.insert(name.to_string(), Atom::Function(function));
        }
    }
    functions
}
