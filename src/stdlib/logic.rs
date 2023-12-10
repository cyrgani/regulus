use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![
        or(),
        and(),
        less(),
        less_equals(),
        greater(),
        greater_equals(),
        not(),
    ]
}

fn bool_cmp_fn_builder(names: &[&str], f: fn(bool, bool) -> bool) -> Function {
    Function::new(
        names,
        Some(2),
        Rc::new(move |storage, args| {
            Ok(Atom::Bool(f(
                args[0].eval(storage)?.bool()?,
                args[1].eval(storage)?.bool()?,
            )))
        }),
    )
}

fn or() -> Function {
    bool_cmp_fn_builder(&["or", "||"], |lhs, rhs| lhs || rhs)
}

fn and() -> Function {
    bool_cmp_fn_builder(&["and", "&&"], |lhs, rhs| lhs && rhs)
}

fn not() -> Function {
    Function::new(
        &["!", "not"],
        Some(1),
        Rc::new(|storage, args| Ok(Atom::Bool(!args[0].eval(storage)?.bool()?))),
	)
}

fn int_cmp_fn_builder(name: &str, f: fn(&i32, &i32) -> bool) -> Function {
    Function::new(
        &[name],
        Some(2),
        Rc::new(move |storage, args| {
            Ok(Atom::Bool(f(
                &args[0].eval(storage)?.int()?,
                &args[1].eval(storage)?.int()?,
            )))
        }),
	)
}

fn less() -> Function {
    int_cmp_fn_builder("<", i32::lt)
}

fn less_equals() -> Function {
    int_cmp_fn_builder("<=", i32::le)
}

fn greater() -> Function {
    int_cmp_fn_builder(">", i32::gt)
}

fn greater_equals() -> Function {
    int_cmp_fn_builder(">=", i32::ge)
}
