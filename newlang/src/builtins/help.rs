use crate::prelude::*;

functions! {
    "help"(1) => |state, args| {
        let arg = args[0].eval(state)?;
        if let Atom::Function(f) = arg {
            Ok(Atom::String(f.doc))
        } else {
            Exception::new_err("`help` must be called on a function", Error::Argument)
        }
        // TODO: 
        //  help will eventually be able to read the doc comment of a function
        //  and print it, among with metadata (argc)
        //Exception::new_err("help not yet implemented", Error::Unimplemented)
    }
}