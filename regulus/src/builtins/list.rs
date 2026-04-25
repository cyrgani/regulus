use crate::builtins::eagerly_evaluate;
use crate::exception::IndexError;
use crate::prelude::*;

fn atom_to_index(atom: &Atom, state: &State) -> Result<usize> {
    usize::try_from(atom.int_e(state)?)
        .map_err(|e| state.raise(IndexError, format!("invalid list index: {e}")))
}

functions! {
    /// Appends the second argument at the back of the list given as first argument and returns
    /// the new list.
    "append"(2) => |state, args| {
        let args = eagerly_evaluate(state, args)?;
        let mut seq = args[0].list_e(state)?;
        seq.push(args[1].clone());
        Ok(Atom::List(seq))
    }
    /// Returns the value in the first list argument at the second integer argument.
    /// Raises an exception if the index is out of bounds.
    "index"(2) => |state, args| {
        let args = eagerly_evaluate(state, args)?;
        let index = atom_to_index(&args[1], state)?;
        args[0]
            .list_e(state)?
            .get(index)
            .cloned()
            .ok_or_else(|| state.raise(IndexError, "sequence index out of bounds"))
    }
    /// Returns the length of the given list argument.
    "len"(1) => |state, args| {
        Atom::int_from_rust_int(args[0].eval_list(state)?.len(), state)
    }
    /// Iterates over the given list elements.
    /// The first argument is the list, the second the loop variable name for each element and the
    /// third is the body that will be run for each of these elements.
    /// Afterwards, `null` is returned.
    // TODO: argument order of seq and loop var is confusing
    "for_in"(3) => |state, args| {
        let v = args[0].eval_list(state)?;
        let loop_var = args[1].variable("invalid loop variable given to `for_in`", state)?;
        let loop_body = &args[2];
        for el in v {
            state.storage.insert(loop_var, el);
            loop_body.eval(state)?;
        }

        Ok(Atom::Null)
    }
    /// Replaces an element at a list index with another.
    /// The first argument is the list, the second the index and the third the new value.
    /// If the index is out of bounds, an exception is raised.
    "replace_at"(3) => |state, args| {
        let args = eagerly_evaluate(state, args)?;
        let mut v = args[0].list_e(state)?;
        let index = atom_to_index(&args[1], state)?;
        *v.get_mut(index).ok_or_else(|| {
            state.raise(IndexError, "unable to insert at index into list")
        })? = args[2].clone();

        Ok(Atom::List(v))
    }
    // TODO: add tests for this
    /// Removes the element at the given list index.
    /// The first argument is the list, the second the index.
    /// If the index is out of bounds, an exception is raised.
    ///
    /// Returns the updated list.
    "remove_at"(2) => |state, args| {
        let args = eagerly_evaluate(state, args)?;
        let mut v = args[0].list_e(state)?;
        let index = atom_to_index(&args[1], state)?;
        v.remove(index);
        Ok(Atom::List(v))
    }
}
