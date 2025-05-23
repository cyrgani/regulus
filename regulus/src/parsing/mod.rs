pub mod positions;
pub mod token;

use crate::parsing::token::Token;
use crate::prelude::*;
pub use token::{TokenData, tokenize, validate_tokens};

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
