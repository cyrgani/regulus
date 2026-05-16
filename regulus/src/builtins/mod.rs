use crate::prelude::*;
use std::collections::HashMap;

mod core;
mod fn_def;
mod import;
mod list;
mod math;
mod private;
mod ty;

pub fn all_functions() -> HashMap<String, Atom> {
    let mut builtins = HashMap::new();

    for module in [
        core::functions(),
        fn_def::functions(),
        private::functions(),
        ty::functions(),
        functions(),
    ] {
        for (name, function) in module {
            builtins.insert(name.to_string(), Atom::Function(function));
        }
    }
    builtins
}

functions! {
    /// Internal function for integer math.
    "__builtin_int_math"(3) => math::builtin_int_math
    /// Internal function for integer bit shifts.
    "__builtin_int_shift"(3) => math::builtin_int_shift
    /// Imports a file, either from the stl or the local directory.
    /// Returns `null`.
    /// TODO document the exact algorithm and hierarchy more clearly, also the return value of this function
    "import"(1) => import::import
    /// Internal function to implement basic list functionality.
    "__builtin_list_api"(_) => list::builtin_list_api
}
