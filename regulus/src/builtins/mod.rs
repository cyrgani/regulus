use crate::prelude::*;
use std::collections::HashMap;

mod core;
mod fn_def;
mod help;
mod import;
mod io;
mod list;
mod math;
mod private;
mod ty;

pub fn all_functions() -> HashMap<String, Atom> {
    let mut functions = HashMap::new();

    for module in [
        core::functions(),
        fn_def::functions(),
        help::functions(),
        import::functions(),
        io::functions(),
        list::functions(),
        math::functions(),
        private::functions(),
        ty::functions(),
    ] {
        for (name, function) in module {
            functions.insert(name.to_string(), Atom::Function(function));
        }
    }
    functions
}
