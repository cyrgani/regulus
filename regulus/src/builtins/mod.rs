use crate::prelude::*;
use std::collections::HashMap;

mod core;
mod fn_def;
mod help;
mod import;
mod io;
mod list;
mod math;
mod private;
mod ty;

pub fn all_functions() -> HashMap<String, Atom> {
    let mut functions = HashMap::new();

    for module in [
        core::functions(),
        fn_def::functions(),
        help::functions(),
        import::functions(),
        io::functions(),
        list::functions(),
        math::functions(),
        private::functions(),
        ty::functions(),
    ] {
        for (name, function) in module {
            functions.insert(name.to_string(), Atom::Function(function));
        }
    }
    functions
}

/// Evaluates all arguments to make a builtin behave just the same as if it was a regular function.
///
/// This should be used for all builtins where this is the expected behavior, such as list builtins.
fn eagerly_evaluate(state: &mut State, args: &[Argument]) -> Result<Vec<Atom>> {
    let mut v = Vec::with_capacity(args.len());
    for arg in args {
        v.push(arg.eval(state)?.into_owned());
    }
    Ok(v)
}
