use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![len()]
}

fn len() -> Function {
    Function {
        aliases: vec![],
        name: String::from("len"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            let len = args[0].eval(storage)?.string()?.len();
            Ok(Atom::Int(len as i32))
        }),
    }
}
