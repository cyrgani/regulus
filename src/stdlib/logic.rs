use crate::{export, function};
use crate::prelude::*;
use crate::stdlib::NamedFunction;

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

fn int_cmp_fn_builder(name: &'static str, f: fn(&i64, &i64) -> bool) -> NamedFunction {
    (
        name,
        Function {
            argc: Some(2),
            callback: Rc::new(move |state, args| {
                Ok(Atom::Bool(f(
                    &args[0].eval(state)?.int()?,
                    &args[1].eval(state)?.int()?,
                )))
            }),
        },
    )
}

fn less() -> NamedFunction {
    int_cmp_fn_builder("<", i64::lt)
}

fn less_equals() -> NamedFunction {
    int_cmp_fn_builder("<=", i64::le)
}

fn greater() -> NamedFunction {
    int_cmp_fn_builder(">", i64::gt)
}

fn greater_equals() -> NamedFunction {
    int_cmp_fn_builder(">=", i64::ge)
}
