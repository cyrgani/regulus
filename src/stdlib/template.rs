use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![run()]
}

fn run() -> Function {
    Function {
        name: String::from("_"),
        argc: None,
        callback: Rc::new(|storage, args| {
            for arg in args {
                arg.eval(storage)?;
            }
            Ok(Atom::Null)
        }),
    }
}
