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
    let arg = build_subprogram(&mut cursor)?;

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
            let r = (&tokens[0..i], &tokens[i]);
            *tokens = &tokens[i + 1..];
            return Ok(r);
        }
    }

    Err(Exception::unspanned(SyntaxError, "missing token"))
}

/// given `_(foo(), bar(baz()))`, this would take `(foo(), bar(baz()))` (with start paren, with end paren)
/// as its argument and return `foo(), bar(baz())` (no start, no end paren).
///
/// sets `tokens` to the tokens in the parens (excluding start and end parens,
/// returns the span of the end paren and the tokens after that.
fn extract_within_parens<'a>(tokens: &mut &'a [Token]) -> Result<(Span, &'a [Token])> {
    // note: tokens[0] will always be a `(`
    let mut stack = 1u32;
    for i in 1..tokens.len() {
        match tokens[i].data {
            TokenData::LeftParen => stack += 1,
            TokenData::RightParen => {
                stack -= 1;
                if stack == 0 {
                    let span = tokens[i].span.clone();
                    let rest = &tokens[i + 1..];
                    *tokens = &tokens[1..i];
                    return Ok((span, rest));
                }
            }
            _ => (),
        }
    }
    syntax_error("unclosed `(` parenthesis", &tokens[0].span)
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

/// returns the constructed argument
fn build_subprogram(tokens: &mut &[Token]) -> Result<Argument> {
    let (doc_comments, first_token) = eat_commented_token(tokens)?;
    if let Some(atom) = first_token.to_atom() {
        return Ok(atom);
    }
    let name = first_token.to_name()?;

    if let Some(Token {
        data: TokenData::LeftParen,
        span: left_paren_span,
    }) = without_comments(tokens).next()
    {
        let (right_paren_span, rest) = extract_within_parens(tokens)?;
        let mut args = vec![];

        while without_comments(tokens).next().is_some() {
            args.push(build_subprogram(tokens)?);

            let Ok((_, comma)) = eat_commented_token(tokens) else {
                break;
            };

            if !comma.is_comma() {
                return syntax_error("missing comma in argument list", &comma.span);
            }
        }
        *tokens = rest;

        Ok(Argument::FunctionCall(
            FunctionCall {
                args,
                name,
                doc_comment: concat_doc_comments(doc_comments),
            },
            Span {
                start: left_paren_span.start,
                ..right_paren_span
            },
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

    #[test]
    fn two_commas() {
        let prog = build_program(tokenize("_(4,,4)", no_path()).unwrap());
        assert_eq!(
            prog.unwrap_err().to_string(),
            "SyntaxError: expected atom or ident\nat <file>:0:5"
        );
    }
}
