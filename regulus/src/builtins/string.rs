use crate::prelude::*;

functions! {
    /// Concatenates any number of strings into one and returns it.
    /// Other values are not implicitly casted and cause an exception.
    "strconcat"(_) => |state, args| {
        let mut string = String::new();
        for arg in args {
            string.push_str(&arg.eval(state)?.string()?);
        }
        Ok(Atom::String(string))
    }
}
