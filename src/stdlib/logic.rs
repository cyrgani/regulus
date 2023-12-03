use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![
        or(),
        and(),
        less(),
        less_equals(),
        greater(),
        greater_equals(),
    ]
}

fn int_cmp_fn_builder(name: &str, f: fn(i32, i32) -> bool) -> Function {
    Function {
        aliases: vec![],
        name: String::from(name),
        argc: Some(2),
        callback: Rc::new(move |program, storage, args| {
            Ok(Atom::Bool(f(
                args[0].eval(program, storage)?.int()?,
                args[1].eval(program, storage)?.int()?,
            )))
        }),
    }
}

fn bool_cmp_fn_builder(name: &str, aliases: Vec<&str>, f: fn(bool, bool) -> bool) -> Function {
    Function {
        aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
        name: String::from(name),
        argc: Some(2),
        callback: Rc::new(move |program, storage, args| {
            Ok(Atom::Bool(f(
                args[0].eval(program, storage)?.bool()?,
                args[1].eval(program, storage)?.bool()?,
            )))
        }),
    }
}

fn or() -> Function {
    bool_cmp_fn_builder("or", vec!["||"], |lhs, rhs| lhs || rhs)
}

fn and() -> Function {
    bool_cmp_fn_builder("and", vec!["&&"], |lhs, rhs| lhs && rhs)
}

fn less() -> Function {
    int_cmp_fn_builder("<", |lhs, rhs| lhs < rhs)
}

fn less_equals() -> Function {
    int_cmp_fn_builder("<=", |lhs, rhs| lhs <= rhs)
}

fn greater() -> Function {
    int_cmp_fn_builder(">", |lhs, rhs| lhs > rhs)
}

fn greater_equals() -> Function {
    int_cmp_fn_builder(">=", |lhs, rhs| lhs >= rhs)
}
