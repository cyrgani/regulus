use crate::prelude::*;

functions! {
    /// Constructs a new list containing all the given arguments.
    "list"(_) => |state, args| {
        let mut list = vec![];
        for arg in args {
            list.push(arg.eval(state)?);
        }
        Ok(Atom::List(list))
    }
    /// Appends the second argument at the back of the list given as first argument and returns
    /// the new list.
    "append"(2) => |state, args| {
        let mut list = args[0].eval(state)?.list()?;
        list.push(args[1].eval(state)?);
        Ok(Atom::List(list))
    }
    /// Takes any amount of lists and joins their elements together into a single list.
    "join"(_) => |state, args| {
        let mut list = vec![];
        for arg in args {
            list.extend(arg.eval(state)?.list()?);
        }
        Ok(Atom::List(list))
    }
    /// Returns the value in the first list argument at the second integer argument.
    /// Raises an exception if the index is out of bounds.
    "index"(2) => |state, args| {
        args[0]
            .eval(state)?
            .list()?
            .get(args[1].eval(state)?.int()? as usize)
            .ok_or_else(|| Exception::new("list index out of bounds", Error::Index))
            .cloned()
    }
    /// Returns the last element of the given list, raising an exception if it is empty.
    "last"(1) => |state, args| {
        args[0]
            .eval(state)?
            .list()?
            .pop()
            .ok_or_else(|| Exception::new("cannot pop from empty list", Error::Index))
    }
    /// Returns the length of the given list argument.
    /// To get the length of a string, use `strlen`.
    "len"(1) => |state, args| {
        Ok(Atom::Int(args[0].eval(state)?.list()?.len() as i64))
    }
    /// Iterates over the given list elements.
    /// The first argument is the list, the second the loop variable name for each element and the 
    /// third is the body that will be run for each of these elements.
    /// Afterwards, `null` is returned.
    /// 
    /// If the loop variable shadows an existing variable, that value can be used again after the loop.
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
    /// Applies the second argument function to each element of the first argument list and returns
    /// the updated list.
    "map"(2) => |state, args| {
        let function = args[1].eval(state)?.function()?;
        let list = args[0].eval(state)?.list()?;
        Ok(Atom::List(
            list.into_iter()
                .map(|atom| (function.callback)(state, &[Argument::Atom(atom)]))
                .collect::<Result<_>>()?,
        ))
    }
    /// Replaces an element at a list index with another.
    /// The first argument is the list, the second the index and the third the new value.
    /// If the index is out of bounds, an exception is raised.
    "overwrite_at_index"(3) => |state, args| {
        let mut list = args[0].eval(state)?.list()?;
        *list
            .get_mut(args[1].eval(state)?.int()? as usize)
            .ok_or_else(|| Exception::new("Unable to insert at index into list!", Error::Index))? =
            args[2].eval(state)?;
        Ok(Atom::List(list))
    }
}
