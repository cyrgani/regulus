use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![list(), push(), index(), pop(), for_each()]
}

fn list() -> Function {
    Function {
        aliases: vec![],
        name: String::from("list"),
        argc: None,
        callback: Rc::new(|state, args| {
            let mut list = vec![];
            for arg in args {
                list.push(arg.eval(state)?);
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
        callback: Rc::new(|state, args| {
            args[0].eval(state)?.list()?.push(args[1].eval(state)?);
            Ok(Atom::Null)
        }),
    }
}

fn index() -> Function {
    Function {
        aliases: vec![],
        name: String::from("index"),
        argc: Some(2),
        callback: Rc::new(|state, args| {
            args[0]
                .eval(state)?
                .list()?
                .get(args[1].eval(state)?.int()? as usize)
                .ok_or_else(|| Exception::new("Unable to index list!", Error::Index))
                .cloned()
        }),
    }
}

fn pop() -> Function {
    Function {
        aliases: vec![],
        name: String::from("pop"),
        argc: Some(1),
        callback: Rc::new(|state, args| {
            args[0]
                .eval(state)?
                .list()?
                .pop()
                .ok_or_else(|| Exception::new("Unable to pop from list!", Error::Index))
        }),
    }
}

fn for_each() -> Function {
    Function {
        aliases: vec![],
        name: String::from("for_each"),
        argc: Some(2),
        callback: Rc::new(|state, args| {
            let function = args[1].eval(state)?.function()?;
            let list = args[0].eval(state)?.list()?;
            for element in list {
                (function.callback)(state, &[Argument::Atom(element.clone())])?;
            }
            Ok(Atom::Null)
        }),
    }
}
