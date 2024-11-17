use crate::function;
use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![list(), push(), index(), pop(), for_each()]
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
    name: for_each,
    argc: Some(2),
    callback: |state, args| {
        let function = args[1].eval(state)?.function()?;
        let list = args[0].eval(state)?.list()?;
        for element in list {
            (function.callback)(state, &[Argument::Atom(element.clone())])?;
        }
        Ok(Atom::Null)
    },
}
