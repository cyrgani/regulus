// The grammar is:
// G = ({S, X}, {a, n, (, ), ","}, P, S)
// (a represents any atom, n any name / ident)
// with P:
//  S -> a | n | n() | n(X)
//  X -> S,X | S, | S

pub mod positions;
pub mod token;

use crate::parsing::positions::Span;
use crate::parsing::token::Token;
use crate::prelude::*;
pub use token::{TokenData, tokenize};

pub fn build_program(mut tokens: Vec<Token>) -> Result<Argument> {
    tokens.retain(|t| !matches!(t.data, TokenData::Comment(_)));
    let (arg, rest) = next_s_step(&tokens)?;

    if !rest.is_empty() {
        return Err(Exception::spanned(
            "trailing unparsed tokens detected",
            Error::Syntax,
            &rest[0].span,
        ));
    }

    Ok(arg)
}

fn get_token(tokens: &[Token], idx: usize) -> Result<&Token> {
    tokens
        .get(idx)
        .ok_or_else(|| Exception::new("missing token", Error::Syntax))
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
            }
            _ => (),
        }
    }
    None
}

/// returns the constructed argument and all remaining tokens
fn next_s_step(tokens: &[Token]) -> Result<(Argument, &[Token])> {
    let first_token = get_token(tokens, 0)?;
    if let Some(atom) = first_token.to_atom() {
        return Ok((Argument::Atom(atom, first_token.span.clone()), &tokens[1..]));
    }
    if let Some(name) = first_token.to_name() {
        // we may not use `?` on the result of `get_token_data`, since that is valid in the `a` or `n` case
        if matches!(
            get_token(tokens, 1).map(|t| &t.data),
            Ok(&TokenData::LeftParen)
        ) {
            if let Some((body, rest)) = find_within_parens(&tokens[2..]) {
                let args = if body.is_empty() {
                    vec![]
                } else {
                    next_x_step(body)?
                };

                return Ok((
                    Argument::FunctionCall(
                        FunctionCall { args, name },
                        Span::new(
                            tokens[1].span.start,
                            tokens.last().unwrap().span.end,
                            tokens[1].span.file.clone(),
                        ),
                    ),
                    rest,
                ));
            }
        } else {
            return Ok((
                Argument::Variable(name, first_token.span.clone()),
                &tokens[1..],
            ));
        }
    }
    // TODO: better error message
    Err(Exception::new(
        "missing or invalid tokens for s_step",
        Error::Syntax,
    ))
}

fn next_x_step(tokens: &[Token]) -> Result<Vec<Argument>> {
    if tokens.is_empty() {
        // TODO: better error message
        return Err(Exception::new("missing tokens for x_step", Error::Syntax));
    }
    let (first_arg, remaining) = next_s_step(tokens)?;
    let mut args = vec![first_arg];
    if remaining.is_empty() {
        return Ok(args);
    }
    if !remaining[0].is_comma() {
        return Err(Exception::spanned(
            "missing comma in argument list",
            Error::Syntax,
            &remaining[0].span,
        ));
    }
    if remaining.len() > 1 {
        args.append(&mut next_x_step(&remaining[1..])?);
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no_path;

    #[test]
    fn extra_parens() {
        let prog = build_program(tokenize("_((2))", no_path()).unwrap());

        assert_eq!(
            prog.unwrap_err(),
            Exception::new("missing or invalid tokens for s_step", Error::Syntax)
        );

        let prog = build_program(tokenize("(print(2)), print(3)", no_path()).unwrap());

        assert_eq!(
            prog.unwrap_err(),
            Exception::new("missing or invalid tokens for s_step", Error::Syntax)
        );
    }
}
