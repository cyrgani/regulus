use crate::prelude::*;

functions! {
    "list"(_) => |state, args| {
        let mut list = vec![];
        for arg in args {
            list.push(arg.eval(state)?);
        }
        Ok(Atom::List(list))
    }
    "push"(2) => |state, args| {
        let mut list = args[0].eval(state)?.list()?;
        list.push(args[1].eval(state)?);
        Ok(Atom::List(list))
    }
    "index"(2) => |state, args| {
        args[0]
            .eval(state)?
            .list()?
            .get(args[1].eval(state)?.int()? as usize)
            .ok_or_else(|| Exception::new("Unable to index list!", Error::Index))
            .cloned()
    }
    "pop"(1) => |state, args| {
        args[0]
            .eval(state)?
            .list()?
            .pop()
            .ok_or_else(|| Exception::new("Unable to pop from list!", Error::Index))
    }
    "len"(1) => |state, args| {
        Ok(Atom::Int(args[0].eval(state)?.list()?.len() as i64))
    }
    "for_in"(3) => |state, args| {
        let list = args[0].eval(state)?.list()?;
        let Argument::Variable(loop_var) = &args[1] else {
            return raise!(Error::Argument, "invalid loop variable given to `for_in`")
        };
        let Argument::FunctionCall(loop_body) = &args[2] else {
            return raise!(Error::Argument, "invalid loop body given to `for_in`")
        };

        let possibly_shadowed_value = state.storage.get(loop_var).cloned();

        for el in list {
            state.storage.insert(loop_var.clone(), el);
            loop_body.eval(state)?;
        }

        if let Some(val) = possibly_shadowed_value {
            state.storage.insert(loop_var.clone(), val);
        }
        Ok(Atom::Null)
    }
    "map"(2) => |state, args| {
        let function = args[1].eval(state)?.function()?;
        let list = args[0].eval(state)?.list()?;
        Ok(Atom::List(
            list.into_iter()
                .map(|atom| (function.callback)(state, &[Argument::Atom(atom)]))
                .collect::<Result<_>>()?,
        ))
    }
    "overwrite_at_index"(3) => |state, args| {
        let mut list = args[0].eval(state)?.list()?;
        *list
            .get_mut(args[1].eval(state)?.int()? as usize)
            .ok_or_else(|| Exception::new("Unable to insert at index into list!", Error::Index))? =
            args[2].eval(state)?;
        Ok(Atom::List(list))
    }
}
