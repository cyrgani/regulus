#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::option_if_let_else,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

mod argument;
mod atom;
mod exception;
mod function;
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
    pub use crate::{
        argument::Argument,
        atom::Atom,
        exception::{Error, Exception, ProgResult},
        function::{Function, FunctionCall},
        run,
        storage::Storage,
    };
    pub use std::rc::Rc;
}

use prelude::*;

fn strip_comments(code: &str) -> String {
    code.split('\n')
        .map(|line| line.split('#').next().unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Function(String),
    LeftParen,
    Comma,
    RightParen,
    Atom(Atom),
    Name(String),
}

fn tokenize(code: &str) -> Vec<Token> {
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
                            Err(()) => Token::Name(current.clone()),
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

    tokens
}

fn validate_tokens(tokens: &[Token]) -> ProgResult<()> {
    let mut left_parens = 0;
    let mut right_parens = 0;

    for token in tokens {
        match token {
            Token::LeftParen => left_parens += 1,
            Token::RightParen => right_parens += 1,
            _ => (),
        }
        if right_parens > left_parens {
            return Exception::new_err(
                format!("More ')' ({right_parens}) than '(' ({left_parens}) at some time!"),
                Error::Syntax,
            );
        }
    }

    if left_parens != right_parens {
        return Exception::new_err(
            format!("Nonequal amount of '(' and ')': {left_parens} vs. {right_parens}"),
            Error::Syntax,
        );
    }

    Ok(())
}

fn build_program(tokens: &[Token], function_name: &str) -> ProgResult<FunctionCall> {
    let mut call = FunctionCall {
        args: vec![],
        name: function_name.to_string(),
    };

    let mut iter = tokens.iter().enumerate();

    while let Some((idx, token)) = iter.next() {
        match token {
            Token::Atom(atom) => call.args.push(Argument::Atom(atom.clone())),
            Token::Comma | Token::LeftParen => (),
            Token::Function(function) => {
                let mut required_right_parens = 1;
                for (i, t) in tokens[idx + 2..].iter().enumerate() {
                    match t {
                        Token::LeftParen => required_right_parens += 1,
                        Token::RightParen => required_right_parens -= 1,
                        _ => (),
                    }
                    if required_right_parens == 0 {
                        call.args.push(Argument::FunctionCall(build_program(
                            &tokens[idx + 2..idx + 3 + i],
                            function,
                        )?));
                        iter.nth(1 + i);
                        break;
                    }
                }
                assert_eq!(
                    required_right_parens, 0,
                    "token validation should cover this"
                );
            }
            Token::Name(name) => call.args.push(Argument::Variable(name.clone())),
            Token::RightParen => return Ok(call),
        }
    }

    Ok(call)
}

pub fn run(code: &str, start_storage: Option<Storage>) -> ProgResult<(Atom, Storage)> {
    let without_comments = strip_comments(code);

    let tokens = tokenize(&without_comments);

    validate_tokens(&tokens)?;

    let program = build_program(&tokens, "_")?;

    let mut storage = start_storage.unwrap_or_else(storage::initial);

    let result = program.eval(&mut storage)?;
    Ok((result, storage))
}
