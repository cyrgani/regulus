use crate::prelude::*;

export! {
    less,
    less_equals,
    greater,
    greater_equals,
    not,
    and,
    or,
}

function! {
    name: or,
    argc: Some(2),
    callback: |state, args| Ok(Atom::Bool(
        args[0].eval(state)?.bool()? ||
        args[1].eval(state)?.bool()?
    )),
}

function! {
    name: and,
    argc: Some(2),
    callback: |state, args| Ok(Atom::Bool(
        args[0].eval(state)?.bool()? &&
        args[1].eval(state)?.bool()?
    )),
}

function! {
    name: not,
    argc: Some(1),
    callback: |state, args| Ok(Atom::Bool(!args[0].eval(state)?.bool()?)),
}

macro_rules! cmp_functions {
    ($(($name: ident, $sym: tt, $cmp: path)),*) => {
        $(function! {
            name: $name,
            override_name: $sym,
            argc: Some(2),
            callback: |state, args| {
                Ok(Atom::Bool($cmp(
                    &args[0].eval(state)?.int()?,
                    &args[1].eval(state)?.int()?,
                )))
            },
        })*
    };
}

cmp_functions! {
    (less, <, i64::lt),
    (less_equals, <=, i64::le),
    (greater, >, i64::gt),
    (greater_equals, >=, i64::ge)
}
