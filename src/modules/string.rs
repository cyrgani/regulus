use crate::{Atom, Function};
use std::rc::Rc;

pub fn functions() -> Vec<Function> {
    vec![len()]
}

fn len() -> Function {
    Function {
        name: String::from("len"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            let len = args[0].eval(program, storage)?.string()?.len();
            Ok(Atom::Int(len as i32))
        }),
    }
}
