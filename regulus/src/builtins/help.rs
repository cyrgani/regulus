use crate::prelude::*;

functions! {
    /// Returns the documentation string for a function.
    /// TODO: implement this for non-builtin functions as well.
    /// TODO: also show the argc somehow
    /// TODO: consider printing directly too
    /// TODO: consider removing the leading newline (should then be done in `functions!`)
    "help"(1) => |state, args| {
        let arg = args[0].eval(state)?;
        if let Atom::Function(f) = arg {
            Ok(Atom::String(f.doc))
        } else {
            raise!(Error::Argument, "`help` must be called on a function")
        }
    }
}
