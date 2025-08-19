use crate::atom::Atom;
use crate::exception::{Exception, Result, SyntaxError};
use crate::parsing::positions::{CharPositions, Position, Span};
use crate::raise;
use std::path::PathBuf;
use std::rc::Rc;
use std::result;

/// A token of source code with location information.
#[derive(Debug)]
pub struct Token {
    /// The actual token.
    pub data: TokenData,
    /// The start and end of the character range this token was created from.
    pub span: Span,
}

impl Token {
    pub(crate) fn to_atom(&self) -> Option<Atom> {
        if let TokenData::Atom(atom) = &self.data {
            Some(atom.clone())
        } else {
            None
        }
    }

    pub(crate) fn to_name(&self) -> Option<String> {
        if let TokenData::Name(name) = &self.data {
            Some(name.to_string())
        } else {
            None
        }
    }

    pub(crate) const fn is_comma(&self) -> bool {
        matches!(self.data, TokenData::Comma)
    }
}

#[derive(Debug)]
pub enum TokenData {
    LeftParen,
    Comma,
    RightParen,
    Atom(Atom),
    Name(String),
    // TODO: use or remove
    #[expect(dead_code)]
    Comment(String),
}

/// Takes characters from the stream until `target` is reached.
/// Returns all characters before `target` and the index of `target`.
/// Returns `Err(all_consumed_chars)` if `target` was never found.
fn take_until(
    chars: impl Iterator<Item = (Position, char)>,
    target: char,
) -> result::Result<(Position, String), String> {
    let mut result = String::new();
    for (pos, c) in chars {
        if c == target {
            return Ok((pos, result));
        }
        result.push(c);
    }
    Err(result)
}

pub fn tokenize(code: &str, file_path: Rc<PathBuf>) -> Result<Vec<Token>> {
    let mut tokens = vec![];

    let mut current = String::new();

    let mut chars = CharPositions::new(code);
    let mut add_token = |data, start, end| {
        tokens.push(Token {
            span: Span::new(start, end, file_path.clone()),
            data,
        });
    };

    let mut current_start_pos = None;

    while let Some((char_pos, c)) = chars.next() {
        match c {
            '(' => {
                if !current.is_empty() {
                    add_token(
                        TokenData::Name(current.clone()),
                        current_start_pos.unwrap(),
                        char_pos.one_back(),
                    );
                    current.clear();
                    current_start_pos = None;
                }
                add_token(TokenData::LeftParen, char_pos, char_pos);
            }
            ')' | ',' | ' ' | '\n' | '\t' => {
                if !current.is_empty() {
                    add_token(
                        match Atom::try_from_str(&current)? {
                            Some(value) => TokenData::Atom(value),
                            None => TokenData::Name(current.clone()),
                        },
                        current_start_pos.unwrap(),
                        char_pos.one_back(),
                    );
                    current.clear();
                    current_start_pos = None;
                }
                add_token(
                    match c {
                        ')' => TokenData::RightParen,
                        ',' => TokenData::Comma,
                        _ => continue,
                    },
                    char_pos,
                    char_pos,
                );
            }
            '"' => {
                let Ok((end_pos, body)) = take_until(chars.by_ref(), '"') else {
                    raise!(SyntaxError, "unclosed string literal");
                };
                add_token(TokenData::Atom(Atom::String(body)), char_pos, end_pos);
            }
            '#' => {
                let (end_pos, body) =
                    take_until(chars.by_ref(), '\n').unwrap_or_else(|body| (last_pos(code), body));
                add_token(TokenData::Comment(body), char_pos, end_pos);
            }
            _ => {
                if current_start_pos.is_none() {
                    current_start_pos = Some(char_pos);
                }
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        add_token(
            match Atom::try_from_str(&current)? {
                Some(value) => TokenData::Atom(value),
                None => TokenData::Name(current.clone()),
            },
            current_start_pos.unwrap(),
            last_pos(code),
        );
    }

    Ok(tokens)
}

fn last_pos(code: &str) -> Position {
    CharPositions::new(code)
        .last()
        .expect("already found some code")
        .0
}

/// Returns all characters of the text that the given span encloses.
/// Returns `None` if the span is invalid (end before start or out of bounds).
// TODO: use or remove
#[cfg_attr(not(test), expect(dead_code))]
pub fn extract(text: &str, span: Span) -> Option<String> {
    let mut start_found = false;

    let mut s = String::new();
    for (pos, c) in CharPositions::new(text) {
        if pos == span.start {
            start_found = true;
        }
        if start_found {
            s.push(c);
        }

        if pos == span.end {
            if !start_found {
                return None;
            }
            return Some(s);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no_path;

    fn sp(start_line: u32, start_col: u32, end_line: u32, end_col: u32) -> Span {
        Span::new(
            Position::new(start_line, start_col),
            Position::new(end_line, end_col),
            no_path(),
        )
    }

    #[expect(clippy::unnecessary_wraps)]
    fn so(text: &str) -> Option<String> {
        Some(text.to_string())
    }

    #[test]
    fn extract_1() {
        let t = "abc\nde\nf\nghi\n";
        assert_eq!(extract(t, sp(1, 1, 1, 4)), so("abc\n"));
        assert_eq!(extract(t, sp(1, 1, 2, 2)), so("abc\nde"));
        assert_eq!(extract(t, sp(1, 1, 2, 1)), so("abc\nd"));
        assert_eq!(extract(t, sp(1, 3, 2, 2)), so("c\nde"));
        assert_eq!(extract(t, sp(1, 1, 1, 2)), so("ab"));
        assert_eq!(extract(t, sp(1, 1, 1, 1)), so("a"));
        assert_eq!(extract(t, sp(1, 1, 1, 1000)), None);
        assert_eq!(extract(t, sp(1, 2, 1, 1)), None);
        assert_eq!(extract(t, sp(2, 2, 3, 2)), so("e\nf\n"));
        assert_eq!(extract(t, sp(3, 1, 1, 1)), None);
        assert_eq!(extract(t, sp(2, 1, 1, 4)), None);
        assert_eq!(extract(t, sp(2, 2, 2, 2)), so("e"));
        assert_eq!(extract(t, sp(3, 2, 3, 2)), so("\n"));
        assert_eq!(extract(t, sp(4, 5, 4, 5)), None);
        assert_eq!(extract(t, sp(4, 5, 4, 4)), None);
        assert_eq!(extract(t, sp(3, 2, 4, 1)), so("\ng"));
        assert_eq!(extract(t, sp(3, 2, 6, 1)), None);
        assert_eq!(extract(t, sp(3, 3, 4, 1)), None);
    }

    #[test]
    fn token_extraction() {
        let code = "_(
	def(double_and_print, x, print(*(2, x))),
)
";
        let tokens = tokenize(code, no_path()).unwrap();

        let parts = tokens
            .into_iter()
            .map(|t| extract(code, t.span).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(
            parts,
            [
                "_",
                "(",
                "def",
                "(",
                "double_and_print",
                ",",
                "x",
                ",",
                "print",
                "(",
                "*",
                "(",
                "2",
                ",",
                "x",
                ")",
                ")",
                ")",
                ",",
                ")"
            ]
            .map(ToString::to_string)
        );

        assert_eq!(parts.join(""), code.replace(['\n', '\t', ' '], ""));
    }
}
