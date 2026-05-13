use crate::prelude::*;

functions! {
    /// Returns the documentation string for a function.
    "doc"(1) => |state, args| {
        Ok(Atom::new_string(args[0].eval_function(state)?.doc()))
    }
    /// Returns the argument count for a function, or `null` if it has none.
    "argc"(1) => |state, args| {
        Ok(if let Some(argc) = args[0].eval_function(state)?.argc() {
            Atom::int_from_rust_int(argc, state)?
        } else {
            Atom::Null
        })
    }
}
