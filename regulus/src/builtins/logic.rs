use crate::prelude::*;

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
    /// Evaluates both arguments as integers and preforms XOR.
    "^"(2) => |state, args| Ok(Atom::Int(args[0].eval_int(state)? ^ args[1].eval_int(state)?))
}
