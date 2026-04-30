use crate::exception::IndexError;
use crate::prelude::*;

const NEW_LIST: i64 = 0;
const LEN: i64 = 1;
const INDEX: i64 = 2;
const REMOVE: i64 = 3;
const INSERT: i64 = 4;

fn builtin_list_api(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let mode = args[0].eval_int(state).unwrap();
    
    let expected_argc = match mode {
        NEW_LIST => 0,
        LEN => 1,
        INDEX | REMOVE => 2,
        INSERT => 3,
        _ => panic!("invalid mode {mode}"),
    };
    assert_eq!(args.len() - 1, expected_argc, "arg mismatch: mode {mode}: expected {expected_argc}, found {}", args.len() - 1);
    
    if mode == NEW_LIST {
        return Ok(Atom::new_list(vec![]));
    }
    
    let mut list = args[1].eval_list(state)?;

    if mode == LEN {
        return Atom::int_from_rust_int(list.len(), state);
    }
     
    let index = usize::try_from(args[2].eval_int(state)?)
        .map_err(|e| state.raise(IndexError, format!("invalid list index: {e}")))?;
    let index_bound = match mode {
        INDEX | REMOVE => list.len(),
        INSERT => list.len() + 1,
        _ => unreachable!(),
    };
    if index >= index_bound {
        raise!(
            state,
            IndexError,
            "index {index} out of bounds for list of len {}",
            list.len()
        );
    }
    
    match mode {
        INDEX => Ok(list[index].clone()),
        REMOVE => {
            list.make_mut().remove(index);
            Ok(Atom::List(list))
        }
        INSERT => {
            let element = args[3].eval(state)?;
            list.make_mut().insert(index, element.into_owned());
            Ok(Atom::List(list))
        }
        _ => unreachable!(),
    }
}

functions! {
    /// Internal function to implement basic list functionality.
    "__builtin_list_api"(_) => |state, args| {
        builtin_list_api(state, args)
    }
}
