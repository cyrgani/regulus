// The grammar is:
// G = ({S, X}, {a, n, (, ), ","}, P, S)
// (a represents any atom, n any name / ident)
// with P:
//  S -> a | n | n() | n(X)
//  X -> S,X | S, | S

pub mod positions;
pub mod token;

use crate::parsing::token::Token;
use crate::prelude::*;
pub use token::{TokenData, tokenize, validate_tokens};

pub fn build_program_v3(mut tokens: Vec<Token>) -> Result<Argument> {
    tokens.retain(|t| !matches!(t.data, TokenData::Comment(_)));
    let (arg, []) = next_s_step(&tokens, &mut 0)? else {
        return Err(Exception::new("trailing unparsed tokens detected", Error::Syntax))
    };
    Ok(arg)
}

fn get_token(tokens: &[Token], idx: usize) -> Result<&Token> {
    tokens
        .get(idx)
        .ok_or_else(|| Exception::new("missing token", Error::Syntax))
}

fn get_token_data(tokens: &[Token], idx: usize) -> Result<&TokenData> {
    get_token(tokens, idx).map(|token| &token.data)
}

/// given `_(foo(), bar(baz()))`, this would take `foo(), bar(baz()))` (no start paren, but with end paren)
/// as its argument and return `foo(), bar(baz())` (no start, no end paren).
/// 
/// returns the tokens in the parens and the rest after them, excluding the start and end parens
fn find_within_parens(tokens: &[Token]) -> Option<(&[Token], &[Token])> {
    let mut stack = 1;
    for (idx, token) in tokens.iter().enumerate() {
        match token.data {
            TokenData::LeftParen => stack += 1,
            TokenData::RightParen => {
                stack -= 1;
                if stack == 0 {
                    return Some((&tokens[0..idx], &tokens[idx + 1..]));
                }
            },
            _ => (),
        }
    }
    None
}

/// returns the constructed argument and all remaining tokens
fn next_s_step<'a>(tokens: &'a [Token], paren_stack_height: &mut usize) -> Result<(Argument, &'a [Token])> {
    if let Some(atom) = get_token(tokens, 0)?.to_atom() {
        // TODO: check appears to be wrong
        /*if tokens.len() > 1 {
            return Err(Exception::new("tokens found after atom", Error::Syntax));
        }*/
        return Ok((atom, &tokens[1..]));
    }
    if let Some(name) = get_token(tokens, 0)?.to_name() {
        if matches!(get_token_data(tokens, 1), Ok(&TokenData::LeftParen)) {
        if let Some((body, rest)) = find_within_parens(&tokens[2..]) {
            
            let args = if body.is_empty() {
                vec![]
            } else {
                next_x_step(body, paren_stack_height)?
            };
            
            // todo: would be better to reuse the name already extracted above
            let name = tokens[0].name().unwrap();
            
            return Ok((Argument {
                data: ArgumentData::FunctionCall(FunctionCall { args, name }),
                span_indices: *tokens[1].indices.start()..=*tokens.last().unwrap().indices.end(),
            }, rest));
        }}else  {
            return Ok((name, &tokens[1..]));
        }
    }
    Err(Exception::new(
        "missing or invalid tokens for s_step",
        Error::Syntax,
    ))
}

fn next_x_step(tokens: &[Token], paren_stack_height: &mut usize) -> Result<Vec<Argument>> {
    if tokens.is_empty() {
        return Err(Exception::new("missing tokens for x_step", Error::Syntax));
    }
    // BUG: we need to give it any number of tokens and then look at the rest
    let (first_arg, remaining) = next_s_step((tokens), paren_stack_height)?;
    let mut args = vec![first_arg];
    if remaining.len() == 0 {
        return Ok(args);
    }
    if !remaining[0].is_comma() {
        return Err(Exception::new("missing comma in argument list", Error::Syntax));
    }
    if remaining.len() > 1 {
        args.append(&mut next_x_step(&remaining[1..], paren_stack_height)?);
    } else {
    }
    Ok(args)
}

pub fn build_program(tokens: &[Token], function_name: &str) -> Result<FunctionCall> {
    let mut call = FunctionCall {
        args: vec![],
        name: function_name.to_string(),
    };

    let mut iter = tokens.iter().enumerate();

    while let Some((idx, token)) = iter.next() {
        match &token.data {
            TokenData::Atom(atom) => call.args.push(Argument {
                data: ArgumentData::Atom(atom.clone()),
                span_indices: token.indices.clone(),
            }),
            TokenData::Comma | TokenData::LeftParen | TokenData::Comment(_) => (),
            TokenData::Function(function) => {
                let mut required_right_parens = 1;
                for (i, t) in tokens[idx + 2..].iter().enumerate() {
                    match t.data {
                        TokenData::LeftParen => required_right_parens += 1,
                        TokenData::RightParen => required_right_parens -= 1,
                        _ => (),
                    }
                    if required_right_parens == 0 {
                        call.args.push(Argument {
                            data: ArgumentData::FunctionCall(build_program(
                                &tokens[idx + 2..idx + 3 + i],
                                function,
                            )?),
                            span_indices: token.indices.clone(),
                        });
                        iter.nth(1 + i);
                        break;
                    }
                }
                assert_eq!(
                    required_right_parens, 0,
                    "token validation should cover this"
                );
            }
            TokenData::Name(name) => call.args.push(Argument {
                data: ArgumentData::Variable(name.clone()),
                span_indices: token.indices.clone(),
            }),
            TokenData::RightParen => return Ok(call),
        }
    }

    Ok(call)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_parens() {
        let prog = build_program(&tokenize("_((2))").unwrap(), "_");

        // TODO: should be a syntax error
        assert_eq!(
            prog,
            Ok(FunctionCall {
                args: vec![Argument {
                    data: ArgumentData::FunctionCall(FunctionCall {
                        args: vec![Argument {
                            data: ArgumentData::Atom(Atom::Int(2)),
                            span_indices: 3..=3
                        }],
                        name: "_".to_string()
                    }),
                    span_indices: 0..=0
                }],
                name: "_".to_string()
            })
        );
    }

    #[test]
    fn extra_parens2() {
        let prog = build_program(&tokenize("(print(2)), print(3)").unwrap(), "_");

        // TODO: should be a syntax error
        assert_eq!(
            prog,
            Ok(FunctionCall {
                args: vec![Argument {
                    data: ArgumentData::FunctionCall(FunctionCall {
                        args: vec![Argument {
                            data: ArgumentData::Atom(Atom::Int(2)),
                            span_indices: 7..=7
                        }],
                        name: "print".to_string()
                    }),
                    span_indices: 1..=5
                }],
                name: "_".to_string()
            })
        );
    }
}
