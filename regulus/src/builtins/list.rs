use crate::builtins::eagerly_evaluate;
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

impl Atom {
    fn string_or_list(&self, state: &State) -> Result<StringOrVec> {
        match self {
            Self::String(s) => Ok(StringOrVec::String(s.clone())),
            Self::List(v) => Ok(StringOrVec::Vec(v.clone())),
            val => raise!(state, TypeError, "{val} should be a string or list"),
        }
    }
}

impl StringOrVec {
    fn into_atom(self) -> Atom {
        match self {
            Self::String(s) => Atom::String(s),
            Self::Vec(v) => Atom::List(v),
        }
    }
}

fn char_to_atom(c: char) -> Atom {
    Atom::String(c.to_string())
}

fn atom_to_index(atom: &Atom, state: &State) -> Result<usize> {
    usize::try_from(atom.int_e(state)?)
        .map_err(|e| state.raise(IndexError, format!("invalid list index: {e}")))
}

functions! {
    /// Appends the second argument at the back of the list given as first argument and returns
    /// the new list.
    /// Alternatively, if the first argument is a string and the second is too, a new concatenated
    /// string will be returned.
    "append"(2) => |state, args| {
        let args = eagerly_evaluate(state, args)?;
        let mut seq = args[0].string_or_list(state)?;
        match &mut seq {
            StringOrVec::String(s) => s.push_str(&args[1].string_e(state)?),
            StringOrVec::Vec(v) => v.push(args[1].clone()),
        }
        Ok(seq.into_atom())
    }
    /// Returns the value in the first list or string argument at the second integer argument.
    /// Raises an exception if the index is out of bounds.
    "index"(2) => |state, args| {
        let args = eagerly_evaluate(state, args)?;
        let index = atom_to_index(&args[1], state)?;
        match args[0].string_or_list(state)? {
            StringOrVec::String(s) => s.chars().nth(index).map(char_to_atom),
            StringOrVec::Vec(v) => v.get(index).cloned(),
        }.ok_or_else(|| state.raise(IndexError, "sequence index out of bounds"))
    }
    /// Returns the length of the given list or string argument.
    "len"(1) => |state, args| {
        Atom::int_from_rust_int(match args[0].eval_string_or_list(state)? {
            StringOrVec::String(s) => s.len(),
            StringOrVec::Vec(v) => v.len(),
        }, state)
    }
    /// Iterates over the given list elements or string characters.
    /// The first argument is the list, the second the loop variable name for each element and the
    /// third is the body that will be run for each of these elements.
    /// Afterwards, `null` is returned.
    // TODO: argument order of seq and loop var is confusing
    "for_in"(3) => |state, args| {
        let seq = args[0].eval_string_or_list(state)?;
        let loop_var = args[1].variable("invalid loop variable given to `for_in`", state)?;
        let loop_body = &args[2];

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

        Ok(Atom::Null)
    }
    /// Replaces an element at a list index with another.
    /// The first argument is the list, the second the index and the third the new value.
    /// If the index is out of bounds, an exception is raised.
    /// If the first argument is a string instead, the new value must be a single character,
    /// otherwise an exception will be raised.
    "replace_at"(3) => |state, args| {
        let args = eagerly_evaluate(state, args)?;
        let mut seq = args[0].string_or_list(state)?;
        let index = atom_to_index(&args[1], state)?;
        match &mut seq {
            StringOrVec::String(s) => {
                let char = args[2].string_e(state)?;
                if char.len() != 1 {
                    raise!(state, IndexError, "atom is not a single character")
                }
                s.replace_range(index..=index, &char);
            }
            StringOrVec::Vec(v) => {
                *v.get_mut(index).ok_or_else(|| {
                    state.raise(IndexError, "Unable to insert at index into list!")
                })? = args[2].clone();
            }
        }

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
        let args = eagerly_evaluate(state, args)?;
        let mut seq = args[0].string_or_list(state)?;
        let index = atom_to_index(&args[1], state)?;
        match &mut seq {
            StringOrVec::String(s) => {
                s.remove(index);
            }
            StringOrVec::Vec(v) => {
                v.remove(index);
            }
        }
        Ok(seq.into_atom())
    }
}
