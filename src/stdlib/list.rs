use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![list(), push(), index(), pop(), for_each()]
}

fn list() -> Function {
    Function {
        name: String::from("list"),
        argc: None,
        callback: Rc::new(|program, storage, args| {
            let mut list = vec![];
            for arg in args {
                list.push(arg.eval(program, storage)?);
            }
            Ok(Atom::List(list))
        }),
    }
}

fn push() -> Function {
    Function {
        name: String::from("push"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            args[0]
                .eval(program, storage)?
                .list()?
                .push(args[1].eval(program, storage)?);
            Ok(Atom::Null)
        }),
    }
}

fn index() -> Function {
    Function {
        name: String::from("index"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            args[0]
                .eval(program, storage)?
                .list()?
                .get(args[1].eval(program, storage)?.int()? as usize)
                .ok_or(ProgError {
                    msg: "Unable to index list!".to_string(),
                    class: IndexError,
                })
                .cloned()
        }),
    }
}

fn pop() -> Function {
    Function {
        name: String::from("pop"),
        argc: Some(1),
        callback: Rc::new(|program, storage, args| {
            args[0]
                .eval(program, storage)?
                .list()?
                .pop()
                .ok_or(ProgError {
                    msg: "Unable to pop from list!".to_string(),
                    class: IndexError,
                })
        }),
    }
}

fn for_each() -> Function {
    Function {
        name: String::from("for_each"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            let function = args[1].eval(program, storage)?.function()?;
            let list = args[0].eval(program, storage)?.list()?;
            for element in list {
                (function.callback)(program, storage, vec![Argument::Atom(element.clone())])?;
            }
            Ok(Atom::Null)
        }),
    }
}
