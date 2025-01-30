use crate::prelude::*;

functions! {
    /// Returns the length of the given string.
    "strlen"(1) => |state, args| {
        let len = args[0].eval(state)?.string()?.len();
        Ok(Atom::Int(len as i64))
    }
    /// Concatenates any number of strings into one and returns it.
    /// Other values are not implicitly cases and cause an exception.
    "strconcat"(_) => |state, args| {
        let mut string = String::new();
        for arg in args {
            string.push_str(&arg.eval(state)?.string()?);
        }
        Ok(Atom::String(string))
    }
}
