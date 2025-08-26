// The grammar is:
// G = ({S, X}, {a, n, (, ), ","}, P, S)
// (a represents any atom, n any name / ident)
// with P:
//  S -> a | n | n() | n(X)
//  X -> S,X | S, | S

pub mod positions;
pub mod token;

use crate::exception::SyntaxError;
use crate::parsing::token::Token;
use crate::prelude::*;
pub use positions::{Position, Span};
pub use token::{TokenData, tokenize};

pub fn build_program(tokens: Vec<Token>) -> Result<Argument> {
    let (arg, rest) = next_s_step(&tokens)?;

    if !is_token_empty(rest) {
        return Err(Exception::spanned(
            SyntaxError,
            "trailing unparsed tokens detected",
            &get_token(rest, 0)?.span,
        ));
    }

    Ok(arg)
}

fn without_comments(tokens: &[Token]) -> impl DoubleEndedIterator<Item = &Token> {
    tokens.iter().filter(|t| !t.is_comment())
}

/// Returns the token with the given index, not counting comments.
fn get_token(tokens: &[Token], idx: usize) -> Result<&Token> {
    without_comments(tokens)
        .nth(idx)
        .ok_or_else(|| Exception::new(SyntaxError, "missing token"))
}

/// Returns all comments before the first non-comment token, then the token itself.
fn get_first_token_and_doc_comments(tokens: &[Token]) -> Result<(&[Token], &Token)> {
    for i in 0..tokens.len() {
        if !tokens[i].is_comment() {
            return Ok((&tokens[0..i], &tokens[i]));
        }
    }

    raise!(SyntaxError, "missing token")
}

fn non_comment_len(tokens: &[Token]) -> usize {
    without_comments(tokens).count()
}

fn is_token_empty(tokens: &[Token]) -> bool {
    without_comments(tokens).next().is_none()
}

/// Returns all the tokens from the given index and beyond, not counting comments before that index.
/// Comments after the index are included.
fn get_tokens_from(mut tokens: &[Token], mut start: usize) -> &[Token] {
    for t in tokens {
        if start == 0 {
            return tokens;
        }
        if !t.is_comment() {
            start -= 1;
        }
        tokens = &tokens[1..];
    }
    &[]
}

/// Returns the last token, not counting comments.
fn get_last_token(tokens: &[Token]) -> Result<&Token> {
    without_comments(tokens)
        .next_back()
        .ok_or_else(|| Exception::new(SyntaxError, "missing token"))
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

/// Takes the given tokens, asserts that they are all comments and
/// returns their concatenated string representation.
fn concat_doc_comments(tokens: &[Token]) -> String {
    let mut s = String::new();
    for t in tokens {
        let TokenData::Comment(doc) = &t.data else {
            unreachable!()
        };
        s.push_str(doc.strip_prefix(' ').unwrap_or(doc));
        s.push('\n');
    }
    s.pop();
    s
}

/// returns the constructed argument and all remaining tokens
fn next_s_step(tokens: &[Token]) -> Result<(Argument, &[Token])> {
    let (doc_comments, first_token) = get_first_token_and_doc_comments(tokens)?;
    if let Some(atom) = first_token.to_atom() {
        return Ok((
            Argument::Atom(atom, first_token.span.clone()),
            get_tokens_from(tokens, 1),
        ));
    }
    if let Some(name) = first_token.to_name() {
        // we may not use `?` on the result of `get_token`, since that is valid in the `a` or `n` case
        if let Ok(token_1) = get_token(tokens, 1)
            && matches!(token_1.data, TokenData::LeftParen)
        {
            if let Some((body, rest)) = find_within_parens(get_tokens_from(tokens, 2)) {
                let args = if is_token_empty(body) {
                    vec![]
                } else {
                    next_x_step(body)?
                };

                return Ok((
                    Argument::FunctionCall(
                        FunctionCall {
                            args,
                            name,
                            doc_comment: concat_doc_comments(doc_comments),
                        },
                        Span::new(
                            token_1.span.start,
                            get_last_token(tokens)?.span.end,
                            token_1.span.file.clone(),
                        ),
                    ),
                    rest,
                ));
            }
        } else {
            return Ok((
                Argument::Variable(name, first_token.span.clone()),
                get_tokens_from(tokens, 1),
            ));
        }
    }
    // TODO: better error message
    Err(Exception::new(
        SyntaxError,
        "missing or invalid tokens for s_step",
    ))
}

fn next_x_step(tokens: &[Token]) -> Result<Vec<Argument>> {
    if is_token_empty(tokens) {
        // TODO: better error message
        return Err(Exception::new(SyntaxError, "missing tokens for x_step"));
    }
    let (first_arg, remaining) = next_s_step(tokens)?;
    let mut args = vec![first_arg];
    if is_token_empty(remaining) {
        return Ok(args);
    }
    if !get_token(remaining, 0)?.is_comma() {
        return Err(Exception::spanned(
            SyntaxError,
            "missing comma in argument list",
            &get_token(remaining, 0)?.span,
        ));
    }
    if non_comment_len(remaining) > 1 {
        args.append(&mut next_x_step(get_tokens_from(remaining, 1))?);
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
            Exception::new(SyntaxError, "missing or invalid tokens for s_step")
        );

        let prog = build_program(tokenize("(print(2)), print(3)", no_path()).unwrap());

        assert_eq!(
            prog.unwrap_err(),
            Exception::new(SyntaxError, "missing or invalid tokens for s_step")
        );
    }
}
