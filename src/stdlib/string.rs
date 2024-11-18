use crate::function;
use crate::prelude::*;
use crate::stdlib::NamedFunction;

pub fn functions() -> Vec<NamedFunction> {
    vec![strlen()]
}

function! {
    name: strlen,
    argc: Some(1),
    callback: |state, args| {
        let len = args[0].eval(state)?.string()?.len();
        Ok(Atom::Int(len as i64))
    },
}
