use crate::exception::IndexError;
use crate::prelude::*;

impl Argument {
    fn eval_index(&self, state: &mut State) -> Result<usize> {
        usize::try_from(self.eval_int(state)?)
            .map_err(|e| state.raise(IndexError, format!("invalid list index: {e}")))
    }
}

functions! {
    /// Insert a value at an index into a list.
    /// Argument order: list, index, element.
    /// The index must be positive and not larger than the length of the list.
    /// That means that inserting at exactly `len(list)` is allowed.
    "insert"(3) => |state, args| {
        let mut list = args[0].eval_list(state)?;
        let index = args[1].eval_index(state)?;
        let element = args[2].eval(state)?;
        list.insert(index, element.into_owned());
        Ok(Atom::List(list))
    }
    /// Returns the value in the first list argument at the second integer argument.
    /// Raises an exception if the index is out of bounds.
    "index"(2) => |state, args| {
        let list = args[0].eval_list(state)?;
        let index = args[1].eval_index(state)?;
        list
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
    // TODO: add tests for this
    /// Removes the element at the given list index.
    /// The first argument is the list, the second the index.
    /// If the index is out of bounds, an exception is raised.
    ///
    /// Returns the updated list.
    "remove_at"(2) => |state, args| {
        let mut v = args[0].eval_list(state)?;
        let index = args[1].eval_index(state)?;
        v.remove(index);
        Ok(Atom::List(v))
    }
}
