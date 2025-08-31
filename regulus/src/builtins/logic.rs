use crate::prelude::*;

fn int_cmp(state: &mut State, args: &[Argument], f: fn(&i64, &i64) -> bool) -> Result<Atom> {
    Ok(Atom::Bool(f(
        &args[0].eval_int(state)?,
        &args[1].eval_int(state)?,
    )))
}

functions! {
    /// Evaluates both arguments as booleans and performs short-circuiting OR on them.
    "||"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval_bool(state)? ||
        args[1].eval_bool(state)?
    ))
    /// Evaluates both arguments as booleans and performs short-circuiting AND on them.
    "&&"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval_bool(state)? &&
        args[1].eval_bool(state)?
    ))
    /// Evaluates the argument as a boolean and performs NOT on it.
    "!"(1) => |state, args| Ok(Atom::Bool(!args[0].eval_bool(state)?))
    // TODO: impl PartialOrd for Atom should be used here?
    //  even if not, these comparisons should work for more than integers (at least for bools)
    /// Evaluates both arguments as integers and checks if the left is less than the right.
    "<"(2) => |state, args| int_cmp(state, args, i64::lt)
    /// Evaluates both arguments as integers and checks if the left is less or equal than the right.
    "<="(2) => |state, args| int_cmp(state, args, i64::le)
    /// Evaluates both arguments as integers and checks if the left is greater than the right.
    ">"(2) => |state, args| int_cmp(state, args, i64::gt)
    /// Evaluates both arguments as integers and checks if the left is greater or equal than the right.
    ">="(2) => |state, args| int_cmp(state, args, i64::ge)
    /// Evaluates both arguments as integers and preforms XOR.
    "^"(2) => |state, args| Ok(Atom::Int(args[0].eval_int(state)? ^ args[1].eval_int(state)?))
}
