use crate::prelude::*;
use std::borrow::Cow;

enum StringOrVec {
    String(String),
    Vec(Vec<Atom>),
}

impl Atom {
    fn string_or_list(&self) -> Result<StringOrVec> {
        match self {
            Self::String(s) => Ok(StringOrVec::String(s.clone())),
            Self::List(v) => Ok(StringOrVec::Vec(v.clone())),
            _ => raise!(Error::Type, "{self} should be a string or list"),
        }
    }
}

fn char_to_atom(c: char) -> Atom {
    Atom::String(c.to_string())
}

impl StringOrVec {
    fn len(&self) -> usize {
        match self {
            Self::String(s) => s.len(),
            Self::Vec(v) => v.len(),
        }
    }

    fn pop(&mut self) -> Option<Atom> {
        match self {
            Self::String(s) => s.pop().map(char_to_atom),
            Self::Vec(v) => v.pop(),
        }
    }

    fn get(&self, index: usize) -> Option<Atom> {
        match self {
            Self::String(s) => s.chars().nth(index).map(char_to_atom),
            Self::Vec(v) => v.get(index).cloned(),
        }
    }
}

#[expect(clippy::needless_pass_by_value, reason = "helper function")]
fn atom_to_index(atom: Cow<'_, Atom>) -> Result<usize> {
    usize::try_from(atom.int()?)
        .map_err(|e| Exception::new(format!("invalid list index: {e}"), Error::Index))
}

functions! {
    /// Constructs a new list containing all the given arguments.
    "list"(_) => |state, args| {
        let mut list = vec![];
        for arg in args {
            list.push(arg.eval(state)?.into_owned());
        }
        Ok(Atom::List(list))
    }
    /// Appends the second argument at the back of the list given as first argument and returns
    /// the new list.
    "append"(2) => |state, args| {
        let mut list = args[0].eval(state)?.list()?;
        list.push(args[1].eval(state)?.into_owned());
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
    /// Returns the value in the first list or string argument at the second integer argument.
    /// Raises an exception if the index is out of bounds.
    "index"(2) => |state, args| {
        args[0]
            .eval(state)?
            .string_or_list()?
            .get(atom_to_index(args[1].eval(state)?)?)
            .ok_or_else(|| Exception::new("list index out of bounds", Error::Index))
    }
    /// Returns the last element of the given list or string, raising an exception if it is empty.
    "last"(1) => |state, args| {
        args[0]
            .eval(state)?
            .string_or_list()?
            .pop()
            .ok_or_else(|| Exception::new("cannot pop from empty list", Error::Index))
    }
    /// Returns the length of the given list or string argument.
    "len"(1) => |state, args| {
        Ok(Atom::Int(
            i64::try_from(args[0].eval(state)?.string_or_list()?.len())
                .map_err(|e| Exception::new(format!("list is too long: {e}"), Error::Overflow))?
        ))
    }
    /// Iterates over the given list elements or string characters.
    /// The first argument is the list, the second the loop variable name for each element and the
    /// third is the body that will be run for each of these elements.
    /// Afterwards, `null` is returned.
    ///
    /// If the loop variable shadows an existing variable, that value can be used again after the loop.
    "for_in"(3) => |state, args| {
        let list = args[0].eval(state)?.string_or_list()?;
        let loop_var = args[1].variable("invalid loop variable given to `for_in`")?;
        let loop_body = args[2].function_call("invalid loop body given to `for_in`")?;

        let possibly_shadowed_value = state.storage.remove(loop_var);

        match list {
            StringOrVec::Vec(v) => for el in v {
                state.storage.insert(loop_var, el);
                loop_body.eval(state)?;
            }
            StringOrVec::String(s) => for el in s.chars() {
                state.storage.insert(loop_var, char_to_atom(el));
                loop_body.eval(state)?;
            }
        }

        if let Some(val) = possibly_shadowed_value {
            state.storage.insert(loop_var, val);
        }
        Ok(Atom::Null)
    }
    /// Replaces an element at a list index with another.
    /// The first argument is the list, the second the index and the third the new value.
    /// If the index is out of bounds, an exception is raised.
    /// TODO: make this also work on strings
    "overwrite_at_index"(3) => |state, args| {
        let mut list = args[0].eval(state)?.list()?;
        *list
            .get_mut(atom_to_index(args[1].eval(state)?)?)
            .ok_or_else(|| Exception::new("Unable to insert at index into list!", Error::Index))? =
            args[2].eval(state)?.into_owned();
        Ok(Atom::List(list))
    }
}
