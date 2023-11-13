use crate::prelude::*;

pub fn functions() -> Vec<Function> {
    vec![add(), subtract(), multiply()]
}

fn add() -> Function {
    Function {
        name: String::from("+"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            match args[0]
                .eval(program, storage)?
                .int()?
                .checked_add(args[1].eval(program, storage)?.int()?)
            {
                Some(i) => Ok(Atom::Int(i)),
                None => Err(ProgError {
                    msg: "overflow occured during addition!".to_string(),
                    class: OverflowError,
                }),
            }
        }),
    }
}

fn subtract() -> Function {
    Function {
        name: String::from("-"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            match args[0]
                .eval(program, storage)?
                .int()?
                .checked_sub(args[1].eval(program, storage)?.int()?)
            {
                Some(i) => Ok(Atom::Int(i)),
                None => Err(ProgError {
                    msg: "overflow occured during subtraction!".to_string(),
                    class: OverflowError,
                }),
            }
        }),
    }
}

fn multiply() -> Function {
    Function {
        name: String::from("*"),
        argc: Some(2),
        callback: Rc::new(|program, storage, args| {
            match args[0]
                .eval(program, storage)?
                .int()?
                .checked_mul(args[1].eval(program, storage)?.int()?)
            {
                Some(i) => Ok(Atom::Int(i)),
                None => Err(ProgError {
                    msg: "overflow occured during multiplication!".to_string(),
                    class: OverflowError,
                }),
            }
        }),
    }
}
