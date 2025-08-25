use crate::exception::ArgumentError;
use crate::prelude::*;

functions! {
    /// Returns the documentation string for a function.
    "doc"(1) => |state, args| {
        let arg = args[0].eval(state)?;
        if let Atom::Function(f) = &*arg {
            Ok(Atom::String(f.doc().to_string()))
        } else {
            raise!(state, ArgumentError, "`doc` must be called on a function")
        }
    }
    /// Returns the argument count for a function, or `null` if it has none.
    "argc"(1) => |state, args| {
        let arg = args[0].eval(state)?;
        if let Atom::Function(f) = &*arg {
            Ok(if let Some(argc) = f.argc() {
                Atom::int_from_rust_int(argc)?
            } else {
                Atom::Null
            })
        } else {
            raise!(state, ArgumentError, "`doc` must be called on a function")
        }
    }
}
