use crate::prelude::*;

export! {
    strlen, strconcat,
}

function! {
    name: strlen,
    argc: Some(1),
    callback: |state, args| {
        let len = args[0].eval(state)?.string()?.len();
        Ok(Atom::Int(len as i64))
    },
}

function! {
    name: strconcat,
    argc: None,
    callback: |state, args| {
        let mut string = String::new();
        for arg in args {
            string.push_str(&arg.eval(state)?.string()?);
        }
        Ok(Atom::String(string))
    },
}
