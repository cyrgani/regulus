use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![list(), push(), index(), pop(), for_each()]
}

fn list() -> Function {
    Function {
        aliases: vec![],
        name: String::from("list"),
        argc: None,
        callback: Rc::new(|storage, args| {
            let mut list = vec![];
            for arg in args {
                list.push(arg.eval(storage)?);
            }
            Ok(Atom::List(list))
        }),
    }
}

fn push() -> Function {
    Function {
        aliases: vec![],
        name: String::from("push"),
        argc: Some(2),
        callback: Rc::new(|storage, args| {
            args[0]
                .eval(storage)?
                .list()?
                .push(args[1].eval(storage)?);
            Ok(Atom::Null)
        }),
    }
}

fn index() -> Function {
    Function {
        aliases: vec![],
        name: String::from("index"),
        argc: Some(2),
        callback: Rc::new(|storage, args| {
            args[0]
                .eval(storage)?
                .list()?
                .get(args[1].eval(storage)?.int()? as usize)
                .ok_or(Exception {
                    msg: "Unable to index list!".to_string(),
                    error: Error::Index,
                })
                .cloned()
        }),
    }
}

fn pop() -> Function {
    Function {
        aliases: vec![],
        name: String::from("pop"),
        argc: Some(1),
        callback: Rc::new(|storage, args| {
            args[0]
                .eval(storage)?
                .list()?
                .pop()
                .ok_or(Exception {
                    msg: "Unable to pop from list!".to_string(),
                    error: Error::Index,
                })
        }),
    }
}

fn for_each() -> Function {
    Function {
        aliases: vec![],
        name: String::from("for_each"),
        argc: Some(2),
        callback: Rc::new(|storage, args| {
            let function = args[1].eval(storage)?.function()?;
            let list = args[0].eval(storage)?.list()?;
            for element in list {
                (function.callback)(storage, &[Argument::Atom(element.clone())])?;
            }
            Ok(Atom::Null)
        }),
    }
}
