// The grammar is:
// G = ({S, X}, {a, n, (, ), ","}, P, S)
// (a represents any atom, n any name / ident)
// with P:
//  S -> a | n | n() | n(X)
//  X -> S,X | S, | S

mod positions;
mod token;

use crate::exception::SyntaxError;
use crate::parsing::token::Token;
use crate::prelude::*;
pub use positions::{Position, Span};
pub(crate) use token::{TokenData, tokenize};

fn syntax_error<T>(msg: impl Into<String>, span: &Span) -> Result<T> {
    Err(Exception::spanned(SyntaxError, msg, span))
}

pub fn build_program(tokens: Vec<Token>) -> Result<Argument> {
    let mut cursor = tokens.as_slice();
    let arg = next_s_step(&mut cursor)?;

    if let Some(t) = without_comments(cursor).next() {
        return syntax_error("trailing unparsed tokens detected", &t.span);
    }

    Ok(arg)
}

fn without_comments(tokens: &[Token]) -> impl DoubleEndedIterator<Item = &Token> {
    tokens.iter().filter(|t| !t.is_comment())
}

/// Returns all comments before the first non-comment token, then the token itself.
/// Moves `tokens` forward, right after the returned token.
fn eat_commented_token<'a>(tokens: &mut &'a [Token]) -> Result<(&'a [Token], &'a Token)> {
    for i in 0..tokens.len() {
        if !tokens[i].is_comment() {
            let r = Ok((&tokens[0..i], &tokens[i]));
            *tokens = &tokens[i + 1..];
            return r;
        }
    }

    Err(Exception::unspanned(SyntaxError, "missing token"))
}

/// given `_(foo(), bar(baz()))`, this would take `(foo(), bar(baz()))` (with start paren, with end paren)
/// as its argument and return `foo(), bar(baz())` (no start, no end paren).
///
/// returns the tokens in the parens (excluding start and end parens), then moves `tokens` forward until right after the end paren.
fn find_within_parens<'a>(tokens: &mut &'a [Token]) -> Option<&'a [Token]> {
    let mut stack = 0;
    for (idx, token) in tokens.iter().enumerate() {
        match token.data {
            TokenData::LeftParen => stack += 1,
            TokenData::RightParen => {
                assert!(stack > 0);
                stack -= 1;
                if stack == 0 {
                    let inner = &tokens[1..idx];
                    *tokens = &tokens[idx + 1..];
                    return Some(inner);
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
fn next_s_step(tokens: &mut &[Token]) -> Result<Argument> {
    let (doc_comments, first_token) = eat_commented_token(tokens)?;
    if let Some(atom) = first_token.to_atom() {
        return Ok(Argument::Atom(atom, first_token.span.clone()));
    }
    let Some(name) = first_token.to_name() else {
        return syntax_error("expected atom or ident", &first_token.span);
    };
    // we may not use `unwrap` here, since that is valid in the `a` or `n` case
    if let Some(token_1) = without_comments(tokens).next()
        && token_1.is_left_paren()
    {
        let Some(mut body) = find_within_parens(tokens) else {
            return syntax_error("unclosed `(` parenthesis", &token_1.span);
        };

        let mut args = vec![];

        loop {
            if without_comments(body).next().is_none() {
                break;
            }
            let first_arg = next_s_step(&mut body)?;
            args.push(first_arg);
            let Ok((_, first)) = eat_commented_token(&mut body) else {
                break;
            };

            if !first.is_comma() {
                return syntax_error("missing comma in argument list", &first.span);
            }
        }

        Ok(Argument::FunctionCall(
            FunctionCall {
                args,
                name,
                doc_comment: concat_doc_comments(doc_comments),
            },
            Span::new(
                token_1.span.start,
                without_comments(tokens)
                    .next_back()
                    .map_or(token_1.span.end, |tok| tok.span.end),
                token_1.span.file.clone(),
            ),
        ))
    } else {
        Ok(Argument::Variable(name, first_token.span.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no_path;

    #[test]
    fn extra_parens() {
        let prog = build_program(tokenize("_((2))", no_path()).unwrap());

        assert_eq!(
            prog.unwrap_err().to_string(),
            "SyntaxError: expected atom or ident\nat <file>:0:3"
        );

        let prog = build_program(tokenize("(print(2)), print(3)", no_path()).unwrap());

        assert_eq!(
            prog.unwrap_err().to_string(),
            "SyntaxError: expected atom or ident\nat <file>:0:1"
        );
    }

    #[test]
    fn atom_fn() {
        let prog = build_program(tokenize("2(4)", no_path()).unwrap());

        assert_eq!(prog.unwrap().stringify(), "2(4)");
    }
}
