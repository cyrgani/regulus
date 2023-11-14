use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![run()]
}

fn run() -> Function {
    Function {
        name: String::from("_"),
        argc: None,
        callback: Rc::new(|program, storage, args| {
            for arg in args {
                arg.eval(program, storage)?;
            }
            Ok(Atom::Null)
        }),
    }
}
