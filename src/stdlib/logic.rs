use crate::prelude::*;
use crate::stdlib::NamedFunction;

pub fn functions() -> Vec<NamedFunction> {
    let mut functions = vec![
        less(),
        less_equals(),
        greater(),
        greater_equals(),
        not(),
        not_excl(),
    ];
    functions.extend(and());
    functions.extend(or());
    functions
}

fn bool_cmp_fn_builder(names: &[&'static str], f: fn(bool, bool) -> bool) -> Vec<NamedFunction> {
    names
        .iter()
        .map(|name| {
            (
                *name,
                Function::new(
                    names,
                    Some(2),
                    Rc::new(move |state, args| {
                        Ok(Atom::Bool(f(
                            args[0].eval(state)?.bool()?,
                            args[1].eval(state)?.bool()?,
                        )))
                    }),
                ),
            )
        })
        .collect()
}

fn or() -> Vec<NamedFunction> {
    bool_cmp_fn_builder(&["or", "||"], |lhs, rhs| lhs || rhs)
}

fn and() -> Vec<NamedFunction> {
    bool_cmp_fn_builder(&["and", "&&"], |lhs, rhs| lhs && rhs)
}

fn not() -> NamedFunction {
    (
        "not",
        Function::new(
            &["!", "not"],
            Some(1),
            Rc::new(|state, args| Ok(Atom::Bool(!args[0].eval(state)?.bool()?))),
        ),
    )
}

fn not_excl() -> NamedFunction {
    (
        "!",
        Function::new(
            &["!", "not"],
            Some(1),
            Rc::new(|state, args| Ok(Atom::Bool(!args[0].eval(state)?.bool()?))),
        ),
    )
}

fn int_cmp_fn_builder(name: &'static str, f: fn(&i64, &i64) -> bool) -> NamedFunction {
    (
        name,
        Function::new(
            &[name],
            Some(2),
            Rc::new(move |state, args| {
                Ok(Atom::Bool(f(
                    &args[0].eval(state)?.int()?,
                    &args[1].eval(state)?.int()?,
                )))
            }),
        ),
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
