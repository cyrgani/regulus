use crate::prelude::*;

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

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut current = String::new();
    let mut in_string = false;

    let mut chars = code.chars();

    while let Some(c) = chars.next() {
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
                '#' => {
                    let mut comment = String::new();
                    for c in chars.by_ref() {
                        comment.push(c);
                        if c == '\n' {
                            break;
                        }
                    }
                    tokens.push(Token::Comment(comment));
                }
                _ => current.push(c),
            }
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

pub fn build_program(tokens: &[Token], function_name: &str) -> ProgResult<FunctionCall> {
    let mut call = FunctionCall {
        args: vec![],
        name: function_name.to_string(),
    };

    let mut iter = tokens.iter().enumerate();

    while let Some((idx, token)) = iter.next() {
        match token {
            Token::Atom(atom) => call.args.push(Argument::Atom(atom.clone())),
            Token::Comma | Token::LeftParen | Token::Comment(_) => (),
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
