use crate::prelude::*;

const LEN: i64 = 1;
const INDEX: i64 = 2;
const REMOVE: i64 = 3;
const INSERT: i64 = 4;

pub fn builtin_list_api(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let mode = args[0].eval_mode(state);

    let expected_argc = match mode {
        LEN => 1,
        INDEX | REMOVE => 2,
        INSERT => 3,
        _ => panic!("invalid mode {mode}"),
    };
    assert_eq!(
        args.len() - 1,
        expected_argc,
        "arg mismatch: mode {mode}: expected {expected_argc}, found {}",
        args.len() - 1
    );

    let mut list = args[1].eval_list(state)?;

    if mode == LEN {
        return Atom::int_from_rust_int(list.len(), state);
    }

    let index = usize::try_from(args[2].eval_int(state)?)
        .map_err(|e| state.raise("Index", format!("invalid list index: {e}")))?;
    let index_bound = match mode {
        INDEX | REMOVE => list.len(),
        INSERT => list.len() + 1,
        _ => unreachable!(),
    };
    if index >= index_bound {
        raise!(
            state,
            "Index",
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
            list.make_mut().insert(index, element);
            Ok(Atom::List(list))
        }
        _ => unreachable!(),
    }
}
