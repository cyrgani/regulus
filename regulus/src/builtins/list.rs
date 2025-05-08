use crate::prelude::*;
use std::borrow::Cow;

enum StringOrVec {
    String(String),
    Vec(Vec<Atom>),
}

enum StrOrSlice<'a> {
    Str(&'a str),
    Slice(&'a [Atom]),
}

impl Atom {
    fn string_or_list(&self) -> Result<StringOrVec> {
        match self {
            Self::String(s) => Ok(StringOrVec::String(s.clone())),
            Self::List(v) => Ok(StringOrVec::Vec(v.clone())),
            _ => raise!(Error::Type, "{self} should be a string or list"),
        }
    }

    fn str_or_slice(&self) -> Result<StrOrSlice> {
        match self {
            Self::String(s) => Ok(StrOrSlice::Str(s)),
            Self::List(v) => Ok(StrOrSlice::Slice(v)),
            _ => raise!(Error::Type, "{self} should be a string or list"),
        }
    }
}

impl StringOrVec {
    fn swap(&mut self, a: usize, b: usize) {
        match self {
            Self::String(s) => {
                let mut chars = s.chars().collect::<Vec<_>>();
                chars.swap(a, b);
                *self = Self::String(chars.into_iter().collect::<String>());
            }
            Self::Vec(v) => v.swap(a, b),
        }
    }

    fn into_atom(self) -> Atom {
        match self {
            Self::String(s) => Atom::String(s),
            Self::Vec(v) => Atom::List(v),
        }
    }
}

impl StrOrSlice<'_> {
    const fn len(&self) -> usize {
        match self {
            Self::Str(s) => s.len(),
            Self::Slice(v) => v.len(),
        }
    }

    fn get(&self, index: usize) -> Option<Atom> {
        match self {
            Self::Str(s) => s.chars().nth(index).map(char_to_atom),
            Self::Slice(v) => v.get(index).cloned(),
        }
    }
}

fn char_to_atom(c: char) -> Atom {
    Atom::String(c.to_string())
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
    ///
    /// If the index does not evalutate to an integer, the first argument will not be evaluated at all.
    "index"(2) => |state, args| {
        let index = atom_to_index(args[1].eval(state)?)?;
        args[0]
            .eval(state)?
            .str_or_slice()?
            .get(index)
            .ok_or_else(|| Exception::new("list index out of bounds", Error::Index))
    }
    /// Returns the length of the given list or string argument.
    "len"(1) => |state, args| {
        Ok(Atom::Int(
            i64::try_from(args[0].eval(state)?.str_or_slice()?.len())
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
    /// TODO: make this also work on strings, give it a better name
    "overwrite_at_index"(3) => |state, args| {
        let mut list = args[0].eval(state)?.list()?;
        *list
            .get_mut(atom_to_index(args[1].eval(state)?)?)
            .ok_or_else(|| Exception::new("Unable to insert at index into list!", Error::Index))? =
            args[2].eval(state)?.into_owned();
        Ok(Atom::List(list))
    }
    /// Swaps the values at two indices of a list or string and returns the new sequence.
    /// The arguments are: list or string, first index, second index.
    ///
    /// The indices may be equal, in which case the returned sequence will not be changed.
    /// If the indices are out of bounds or invalid, an exception is raised.
    "swap"(3) => |state, args| {
        let mut seq = args[0].eval(state)?.string_or_list()?;
        seq.swap(atom_to_index(args[1].eval(state)?)?, atom_to_index(args[2].eval(state)?)?);
        Ok(seq.into_atom())
    }
}
