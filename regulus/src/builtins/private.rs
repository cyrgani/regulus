//! Builtin functions which will never have a stable equivalent and are for internal use only.
use crate::prelude::*;

functions! {
    /// Prints the debug representation of the given argument to stdout, followed by a newline.
    ///
    /// NOTE: the output format of this method is unstable.
    /// NOTE: this method may be removed in the future.
    "__builtin_rust_debug"(1) => |state, args| {
        let arg_val = args[0].eval(state)?;
        let s = format!("{arg_val:?}\n");
        state.write_to_stdout(&s);
        Ok(Atom::Null)
    }
    /// TODO
    "__builtin_prelude_import"(0) => |state, _| {
        Ok(Atom::Null)
    }
}
