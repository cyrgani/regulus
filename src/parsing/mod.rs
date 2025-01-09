pub mod token;

use crate::prelude::*;
pub use token::{tokenize, validate_tokens, Token};

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
