use crate::{Atom, Function};
use std::rc::Rc;

pub fn functions() -> Vec<Function> {
    vec![debug()]
}

fn debug() -> Function {
    Function {
        name: String::from("debug"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            println!("{:?}", args[0].eval(program, storage));
            Ok(Atom::Null)
        }),
    }
}
