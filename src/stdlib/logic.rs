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

fn _int_cmp_fn(name: &str, f: fn(i32, i32) -> bool) -> Function {
    Function {
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

fn _bool_cmp_fn(name: &str, f: fn(bool, bool) -> bool) -> Function {
    Function {
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
	_bool_cmp_fn("or", |lhs, rhs| lhs || rhs)
}

fn and() -> Function {
	_bool_cmp_fn("and", |lhs, rhs| lhs && rhs)
}

fn less() -> Function {
    _int_cmp_fn("<", |lhs, rhs| lhs < rhs)
}

fn less_equals() -> Function {
	_int_cmp_fn("<=", |lhs, rhs| lhs <= rhs)
}

fn greater() -> Function {
    _int_cmp_fn(">", |lhs, rhs| lhs > rhs)
}

fn greater_equals() -> Function {
	_int_cmp_fn(">=", |lhs, rhs| lhs >= rhs)
}
