mod argument;
mod atom;
mod function;
mod progerror;
mod storage;

mod stdlib {
    pub mod cast;
    pub mod core;
    pub mod io;
    pub mod list;
    pub mod logic;
    pub mod math;
    pub mod string;
}

pub mod prelude {
    pub use crate::argument::Argument;
    pub use crate::atom::Atom;
    pub use crate::function::{Function, FunctionCall};
    pub use crate::progerror::{ErrorClass::*, ProgError, ProgResult};
    pub use std::rc::Rc;
    pub use crate::storage::Storage;
	pub use crate::run;
}

use prelude::*;

#[derive(Debug, Clone)]
enum Token {
    Function(String),
    LeftParen,
    Comma,
    RightParen,
    Atom(Atom),
    Name(String),
}

fn tokenize(code: &str) -> ProgResult<Vec<Token>> {
    let mut tokens = vec![];
    let mut current = String::new();
    let mut in_string = false;

    for c in code.chars() {
        if in_string {
            match c {
                '"' => {
                    tokens.push(Token::Atom(Atom::String(current.clone())));
                    current.clear();
                    in_string = !in_string;
                }
                _ => current.push(c),
            }
        } else {
            match c {
                '(' => {
                    if !current.is_empty() {
                        tokens.push(Token::Function(current.clone()));
                        current.clear();
                    }
                    tokens.push(Token::LeftParen);
                }
                ')' | ',' => {
                    if !current.is_empty() {
                        tokens.push(match Atom::try_from(current.as_str()) {
                            Ok(value) => Token::Atom(value),
                            Err(_) => Token::Name(current.clone()),
                        });
                        current.clear();
                    }
                    tokens.push(match c {
                        ')' => Token::RightParen,
                        ',' => Token::Comma,
                        _ => unreachable!(),
                    });
                }
                '"' => {
                    in_string = !in_string;
                }
                ' ' | '\n' | '\t' => (),
                _ => current.push(c),
            }
        }
    }

    Ok(tokens)
}

fn build_program(tokens: &[Token]) -> ProgResult<Vec<Argument>> {
    let mut program = vec![];

    let mut current = None;

    for token in tokens {
        let mut new = |arg| {
            let new_id = program.len();
            program.push(arg);
            if let Some(parent) = current {
                if let Argument::FunctionCall(call) = &mut program[parent] {
                    call.arg_locations.push(new_id)
                }
            }
        };

        match token {
            Token::Function(f) => new(Argument::FunctionCall(FunctionCall {
                arg_locations: vec![],
                parent: current,
                name: f.clone(),
            })),
            Token::LeftParen => match program.len() {
                0 => {
                    return Err(ProgError {
                        msg: "found `LeftParen` without existing function!".to_string(),
                        class: SyntaxError,
                    })
                }
                _ => current = Some(program.len() - 1),
            },
            Token::Comma => (), // TODO?
            Token::RightParen => match current {
                Some(i) => {
                    if let Argument::FunctionCall(call) = &program[i] {
                        current = call.parent
                    }
                }
                None => {
                    return Err(ProgError {
                        msg: "found `RightParen` without existing function!".to_string(),
                        class: SyntaxError,
                    })
                }
            },
            Token::Name(name) => new(Argument::Variable(name.clone())),
            Token::Atom(atom) => new(Argument::Atom(atom.clone())),
        };
    }

    Ok(program)
}

fn strip_comments(code: &str) -> String {
    code.split('\n')
        .map(|line| line.split('#').next().unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn run(code: &str, start_storage: Option<Storage>) -> ProgResult<(Atom, Storage)> {
    let without_comments = strip_comments(code);

    let tokens = tokenize(&without_comments)?;

    let program = build_program(&tokens)?;

    let mut storage = start_storage.unwrap_or_else(storage::initial_storage);

    let result = program
        .first()
        .ok_or(ProgError {
            msg: "No function was found!".to_string(),
            class: SyntaxError,
        })?
        .eval(&program, &mut storage)?;
    Ok((result, storage))
}
