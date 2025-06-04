use crate::prelude::*;

functions! {
    /// Returns the documentation string for a function.
    ///
    /// TODO: implement this for non-builtin functions as well.
    "doc"(1) => |state, args| {
        let arg = args[0].eval(state)?;
        if let Atom::Function(f) = &*arg {
            Ok(Atom::String(f.doc().to_string()))
        } else {
            raise!(Error::Argument, "`doc` must be called on a function")
        }
    }

    /// Prints the documentation string for a function as well its argc.
    ///
    /// Use `doc(1)` to return it instead.
    ///
    /// TODO (not yet possible): also show the name somehow
    "help"(1) => |state, args| {
        if let Atom::Function(f) = &*args[0].eval(state)? {
            let argc_str = match f.argc() {
                Some(argc) => argc.to_string(),
                None => "_".to_string(),
            };
            let msg = format!("<function>({argc_str}): \n\n{}\n", f.doc());
            state.write_to_stdout(&msg);
        } else {
            raise!(Error::Argument, "`doc` must be called on a function")
        }
        Ok(Atom::Null)
    }
}
