use crate::prelude::*;

fn int_cmp(state: &mut State, args: &[Argument], f: fn(&i64, &i64) -> bool) -> Result<Atom> {
    Ok(Atom::Bool(f(
        &args[0].eval(state)?.int()?,
        &args[1].eval(state)?.int()?,
    )))
}

functions! {
    // TODO: all functions except `!` need quotes, `!` has them for consistency
    "||"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval(state)?.bool()? ||
        args[1].eval(state)?.bool()?
    ))
    "&&"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval(state)?.bool()? &&
        args[1].eval(state)?.bool()?
    ))
    "!"(1) => |state, args| Ok(Atom::Bool(!args[0].eval(state)?.bool()?))
    "<"(2) => |state, args| int_cmp(state, args, i64::lt)
    "<="(2) => |state, args| int_cmp(state, args, i64::le)
    ">"(2) => |state, args| int_cmp(state, args, i64::gt)
    ">="(2) => |state, args| int_cmp(state, args, i64::ge)
}
