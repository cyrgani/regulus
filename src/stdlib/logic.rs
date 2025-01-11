use crate::prelude::*;

fn int_cmp(state: &mut State, args: &[Argument], f: fn(&i64, &i64) -> bool) -> ProgResult<Atom> {
    Ok(Atom::Bool(f(
        &args[0].eval(state)?.int()?,
        &args[1].eval(state)?.int()?,
    )))
}

functions! {
    "||"(2) => |state, args| Ok(Atom::Bool(
        args[0].eval(state)?.bool()? ||
        args[1].eval(state)?.bool()?
    ))
    // TODO: `&&` is the reason why this must be a quoted name invocation
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
