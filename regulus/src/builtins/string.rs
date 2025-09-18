use crate::prelude::*;

functions! {
    /// Returns a string consisting of one newline character.
    "endl"(0) => |_, _| {
        Ok(Atom::String("\n".to_string()))
    }
}
