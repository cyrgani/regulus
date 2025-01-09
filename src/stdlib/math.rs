use crate::prelude::*;

export! {
    add, subtract, multiply, divide, modulo,
}

macro_rules! aritmethic_functions {
    ($(($name: ident, $sym: tt, $op_name: literal, $cmp: path)),*) => {
        $(function! {
            name: $name,
            override_name: $sym,
            argc: Some(2),
            callback: |state, args| {
                match $cmp(args[0].eval(state)?.int()?, args[1].eval(state)?.int()?) {
                    Some(i) => Ok(Atom::Int(i)),
                    None => Exception::new_err(
                        format!("overflow occured during {}!", $op_name),
                        Error::Overflow,
                    ),
                }
            },
        })*
    };
}

aritmethic_functions! {
    (add, +, "addition", i64::checked_add),
    (subtract, -, "subtraction", i64::checked_sub),
    (multiply, *, "multiplication", i64::checked_mul),
    (divide, /, "division", i64::checked_div),
    (modulo, %, "modulo", i64::checked_rem)
}
