use std::path::PathBuf;
use crate::atom::Atom;
use crate::exception::{Error, Exception, ProgResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Function(String),
    LeftParen,
    Comma,
    RightParen,
    Atom(Atom),
    Name(String),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub file: PathBuf,
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Takes characters from the stream until `target` is reached, returns all characters before `target`.
fn take_until(chars: impl Iterator<Item = char>, target: char) -> String {
    let mut result = String::new();
    for c in chars {
        if c == target {
            break;
        }
        result.push(c);
    }
    result
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut current = String::new();

    let mut chars = code.chars();

    while let Some(c) = chars.next() {
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
                tokens.push(Token::Atom(Atom::String(take_until(chars.by_ref(), '"'))));
            }
            ' ' | '\n' | '\t' => (),
            '#' => {
                tokens.push(Token::Comment(take_until(chars.by_ref(), '\n')));
            }
            _ => current.push(c),
        }
    }

    tokens
}

pub fn validate_tokens(tokens: &[Token]) -> ProgResult<()> {
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
