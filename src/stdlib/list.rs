use crate::prelude::*;

export! {
    list, push, index, pop, for_in, len, map,
}

function! {
    name: list,
    argc: None,
    callback: |state, args| {
        let mut list = vec![];
        for arg in args {
            list.push(arg.eval(state)?);
        }
        Ok(Atom::List(list))
    },
}

function! {
    name: push,
    argc: Some(2),
    callback: |state, args| {
        args[0].eval(state)?.list()?.push(args[1].eval(state)?);
        Ok(Atom::Null)
    },
}

function! {
    name: index,
    argc: Some(2),
    callback: |state, args| {
        args[0]
            .eval(state)?
            .list()?
            .get(args[1].eval(state)?.int()? as usize)
            .ok_or_else(|| Exception::new("Unable to index list!", Error::Index))
            .cloned()
    },
}

function! {
    name: pop,
    argc: Some(1),
    callback: |state, args| {
        args[0]
            .eval(state)?
            .list()?
            .pop()
            .ok_or_else(|| Exception::new("Unable to pop from list!", Error::Index))
    },
}

function! {
    name: len,
    argc: Some(1),
    callback: |state, args| {
        Ok(Atom::Int(args[0].eval(state)?.list()?.len() as i64))
    },
}

function! {
    name: for_in,
    argc: Some(3),
    callback: |state, args| {
        let list = args[0].eval(state)?.list()?;
        let Argument::Variable(loop_var) = &args[1] else {
            return Exception::new_err("invalid loop variable given to `for_in`", Error::Argument)
        };
        let Argument::FunctionCall(loop_body) = &args[2] else {
            return Exception::new_err("invalid loop body given to `for_in`", Error::Argument)
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
    },
}

function! {
    name: map,
    argc: Some(2),
    callback: |state, args| {
        let function = args[1].eval(state)?.function()?;
        let list = args[0].eval(state)?.list()?;
        Ok(Atom::List(
            list.into_iter()
                .map(|atom| (function.callback)(state, &[Argument::Atom(atom)]))
                .collect::<Result<_, _>>()?,
        ))
    },
}
