use crate::prelude::Atom;
use std::collections::HashMap;

macro_rules! builtin_modules {
    ($($name: ident),*) => {
        $(pub mod $name;)*

        pub fn all_functions() -> HashMap<String, Atom> {
            let mut functions = HashMap::new();

            for module in [$($name::functions()),*] {
                for (name, function) in module {
                    functions.insert(name.to_string(), Atom::Function(function));
                }
            }

            functions
        }
    };
}

builtin_modules! {
    cast, core, help, io, list, logic, math, private, string, time, ty
}
