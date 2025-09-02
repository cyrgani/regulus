use crate::exception::{IndexError, TypeError};
use crate::prelude::*;

enum StringOrVec {
    String(String),
    Vec(Vec<Atom>),
}

impl Argument {
    fn eval_string_or_list(&self, state: &mut State) -> Result<StringOrVec> {
        match self.eval(state)?.into_owned() {
            Atom::String(s) => Ok(StringOrVec::String(s)),
            Atom::List(v) => Ok(StringOrVec::Vec(v)),
            val => raise!(state, TypeError, "{val} should be a string or list"),
        }
    }
}

impl StringOrVec {
    fn replace_at(&mut self, index: usize, arg: &Argument, state: &mut State) -> Result<()> {
        match self {
            Self::String(s) => {
                let char = arg.eval_string(state)?;
                if char.len() != 1 {
                    raise!(state, IndexError, "atom is not a single character")
                }
                s.replace_range(index..=index, &char);
            }
            Self::Vec(v) => {
                *v.get_mut(index).ok_or_else(|| {
                    state.raise(IndexError, "Unable to insert at index into list!")
                })? = arg.eval(state)?.into_owned();
            }
        }
        Ok(())
    }

    fn remove_at(&mut self, index: usize) {
        match self {
            Self::String(s) => {
                s.remove(index);
            }
            Self::Vec(v) => {
                v.remove(index);
            }
        }
    }

    fn into_atom(self) -> Atom {
        match self {
            Self::String(s) => Atom::String(s),
            Self::Vec(v) => Atom::List(v),
        }
    }

    fn push(&mut self, arg: &Argument, state: &mut State) -> Result<()> {
        match self {
            Self::String(s) => {
                s.push_str(&arg.eval_string(state)?);
            }
            Self::Vec(v) => {
                v.push(arg.eval(state)?.into_owned());
            }
        }
        Ok(())
    }

    const fn len(&self) -> usize {
        match self {
            Self::String(s) => s.len(),
            Self::Vec(v) => v.len(),
        }
    }

    fn get(&self, index: usize) -> Option<Atom> {
        match self {
            Self::String(s) => s.chars().nth(index).map(char_to_atom),
            Self::Vec(v) => v.get(index).cloned(),
        }
    }
}

fn char_to_atom(c: char) -> Atom {
    Atom::String(c.to_string())
}

fn atom_to_index(arg: &Argument, state: &mut State) -> Result<usize> {
    usize::try_from(arg.eval_int(state)?)
        .map_err(|e| state.raise(IndexError, format!("invalid list index: {e}")))
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
    /// Alternatively, if the first argument is a string and the second is too, a new concatenated
    /// string will be returned.
    "append"(2) => |state, args| {
        let mut seq = args[0].eval_string_or_list(state)?;
        seq.push(&args[1], state)?;
        Ok(seq.into_atom())
    }
    /// Returns the value in the first list or string argument at the second integer argument.
    /// Raises an exception if the index is out of bounds.
    ///
    /// If the index does not evalutate to an integer, the first argument will not be evaluated at all.
    "index"(2) => |state, args| {
        let index = atom_to_index(&args[1], state)?;
        args[0]
            .eval_string_or_list(state)?
            .get(index)
            .ok_or_else(|| state.raise(IndexError, "sequence index out of bounds"))
    }
    /// Returns the length of the given list or string argument.
    "len"(1) => |state, args| {
        Atom::int_from_rust_int(args[0].eval_string_or_list(state)?.len(), state)
    }
    /// Iterates over the given list elements or string characters.
    /// The first argument is the list, the second the loop variable name for each element and the
    /// third is the body that will be run for each of these elements.
    /// Afterwards, `null` is returned.
    ///
    /// If the loop variable shadows an existing variable, that value can be used again after the loop.
    // TODO: argument order of seq and loop var is confusing
    "for_in"(3) => |state, args| {
        let seq = args[0].eval_string_or_list(state)?;
        let loop_var = args[1].variable("invalid loop variable given to `for_in`", state)?;
        let loop_body = &args[2];

        let possibly_shadowed_value = state.storage.remove(loop_var);

        match seq {
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
    /// If the first argument is a string instead, the new value must be a single character,
    /// otherwise an exception will be raised.
    "replace_at"(3) => |state, args| {
        let mut seq = args[0].eval_string_or_list(state)?;
        seq.replace_at(atom_to_index(&args[1], state)?, &args[2], state)?;
        Ok(seq.into_atom())
    }
    // TODO: add tests for this
    /// Removes the element at the given list index.
    /// The first argument is the list, the second the index.
    /// If the index is out of bounds, an exception is raised.
    /// If the first argument is a string instead, the single character at that position will be removed.
    ///
    /// Returns the updated sequence.
    "remove_at"(2) => |state, args| {
        let mut seq = args[0].eval_string_or_list(state)?;
        seq.remove_at(atom_to_index(&args[1], state)?);
        Ok(seq.into_atom())
    }
}
