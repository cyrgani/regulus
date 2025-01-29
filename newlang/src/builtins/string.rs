use crate::prelude::*;

functions! {
    strlen(1) => |state, args| {
        let len = args[0].eval(state)?.string()?.len();
        Ok(Atom::Int(len as i64))
    }
    strconcat(_) => |state, args| {
        let mut string = String::new();
        for arg in args {
            string.push_str(&arg.eval(state)?.string()?);
        }
        Ok(Atom::String(string))
    }
}
