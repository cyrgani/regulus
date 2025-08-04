use crate::atom::Atom;
use crate::exception::{Error, Exception, Result};
use crate::parsing::positions::{CharPositions, Position, Span};
use crate::raise;
use std::path::PathBuf;
use std::rc::Rc;
use std::result;

/// A token of source code with location information.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
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
#[cfg_attr(test, derive(PartialEq))]
pub enum TokenData {
    LeftParen,
    Comma,
    RightParen,
    Atom(Atom),
    Name(String),
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

    let mut current_start_idx = None;

    while let Some((char_idx, c)) = chars.next() {
        match c {
            '(' => {
                if !current.is_empty() {
                    add_token(
                        TokenData::Name(current.clone()),
                        current_start_idx.unwrap(),
                        char_idx.one_back(),
                    );
                    current.clear();
                    current_start_idx = None;
                }
                add_token(TokenData::LeftParen, char_idx, char_idx);
            }
            ')' | ',' | ' ' | '\n' | '\t' => {
                if !current.is_empty() {
                    add_token(
                        match Atom::try_from_str(current.as_str())? {
                            Some(value) => TokenData::Atom(value),
                            None => TokenData::Name(current.clone()),
                        },
                        current_start_idx.unwrap(),
                        char_idx.one_back(),
                    );
                    current.clear();
                    current_start_idx = None;
                }
                add_token(
                    match c {
                        ')' => TokenData::RightParen,
                        ',' => TokenData::Comma,
                        _ => continue,
                    },
                    char_idx,
                    char_idx,
                );
            }
            '"' => {
                let Ok((end_pos, body)) = take_until(chars.by_ref(), '"') else {
                    raise!(Error::Syntax, "unclosed string literal");
                };
                add_token(TokenData::Atom(Atom::String(body)), char_idx, end_pos);
            }
            '#' => {
                let (end_pos, body) =
                    take_until(chars.by_ref(), '\n').unwrap_or_else(|body| (last_pos(code), body));
                add_token(TokenData::Comment(body), char_idx, end_pos);
            }
            _ => {
                if current_start_idx.is_none() {
                    current_start_idx = Some(char_idx);
                }
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        add_token(
            match Atom::try_from_str(current.as_str())? {
                Some(value) => TokenData::Atom(value),
                None => TokenData::Name(current.clone()),
            },
            current_start_idx.unwrap(),
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

// TODO: reenable or remove
#[cfg(false)]
/// Returns all characters of the text that the given indices enclose.
/// Returns `None` if the indices are invalid (end before start or out of bounds).
pub fn extract(text: &str, span: Span) -> Option<String> {
    if span.start > span.end || span.end as usize >= text.chars().count() {
        return None;
    }

    Some(
        text.chars()
            .skip(span.start as usize)
            .take(span.len() as usize)
            .collect::<String>(),
    )
}

// TODO: reenable or remove
#[cfg(false)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::no_path;

    fn sp(start: u32, end: u32) -> Span {
        Span::new(start, end, no_path())
    }

    #[expect(clippy::unnecessary_wraps)]
    fn so(text: &str) -> Option<String> {
        Some(text.to_string())
    }

    #[test]
    fn extract_1() {
        let t = "abc\nde\nf\n";
        assert_eq!(extract(t, sp(0, 3)), so("abc\n"));
        assert_eq!(extract(t, sp(0, 5)), so("abc\nde"));
        assert_eq!(extract(t, sp(0, 4)), so("abc\nd"));
        assert_eq!(extract(t, sp(2, 5)), so("c\nde"));
        assert_eq!(extract(t, sp(0, 1)), so("ab"));
        assert_eq!(extract(t, sp(0, 0)), so("a"));
        assert_eq!(extract(t, sp(0, 1000)), None);
        assert_eq!(extract(t, sp(2, 1)), None);
        assert_eq!(extract(t, sp(5, 8)), so("e\nf\n"));
        assert_eq!(extract(t, sp(5, 0)), None);
        assert_eq!(extract(t, sp(5, 4)), None);
        assert_eq!(extract(t, sp(5, 5)), so("e"));
        assert_eq!(extract(t, sp(8, 8)), so("\n"));
        assert_eq!(extract(t, sp(9, 9)), None);
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

    /*
    use TokenData::*;

    #[expect(non_snake_case)]
    fn Int(val: i64) -> crate::atom::Atom {
        crate::atom::Atom::Int(val)
    }

    #[expect(non_snake_case)]
    fn Name(name: &str) -> TokenData {
        TokenData::Name(name.to_string())
    }

    #[test]
    fn extra_parens() {
        assert_eq!(
            tokenize("_((2))"),
            Ok(vec![
                Token {
                    data: Name("_"),
                    indices: 0..=0
                },
                Token {
                    data: LeftParen,
                    indices: 1..=1
                },
                Token {
                    data: LeftParen,
                    indices: 2..=2
                },
                Token {
                    data: Atom(Int(2)),
                    indices: 3..=3
                },
                Token {
                    data: RightParen,
                    indices: 4..=4
                },
                Token {
                    data: RightParen,
                    indices: 5..=5
                }
            ])
        );
    }

    #[test]
    fn inline_whitespace() {
        assert_eq!(
            tokenize("_(2 3)"),
            Ok(vec![
                Token {
                    data: Name("_"),
                    indices: 0..=0
                },
                Token {
                    data: LeftParen,
                    indices: 1..=1
                },
                Token {
                    data: Atom(Int(2)),
                    indices: 2..=2
                },
                Token {
                    data: Atom(Int(3)),
                    indices: 4..=4
                },
                Token {
                    data: RightParen,
                    indices: 5..=5
                }
            ])
        );
        assert_eq!(
            tokenize("=(a a, 3)"),
            Ok(vec![
                Token {
                    data: Name("="),
                    indices: 0..=0
                },
                Token {
                    data: LeftParen,
                    indices: 1..=1
                },
                Token {
                    data: Name("a"),
                    indices: 2..=2
                },
                Token {
                    data: Name("a"),
                    indices: 4..=4
                },
                Token {
                    data: Comma,
                    indices: 5..=5
                },
                Token {
                    data: Atom(Int(3)),
                    indices: 7..=7
                },
                Token {
                    data: RightParen,
                    indices: 8..=8
                }
            ])
        );
    }

    #[test]
    fn extra_parens_2() {
        assert_eq!(
            tokenize("(print(2)), print(3)"),
            Ok(vec![
                Token {
                    data: LeftParen,
                    indices: 0..=0
                },
                Token {
                    data: Name("print"),
                    indices: 1..=5
                },
                Token {
                    data: LeftParen,
                    indices: 6..=6
                },
                Token {
                    data: Atom(Int(2)),
                    indices: 7..=7
                },
                Token {
                    data: RightParen,
                    indices: 8..=8
                },
                Token {
                    data: RightParen,
                    indices: 9..=9
                },
                Token {
                    data: Comma,
                    indices: 10..=10
                },
                Token {
                    data: Name("print"),
                    indices: 12..=16
                },
                Token {
                    data: LeftParen,
                    indices: 17..=17
                },
                Token {
                    data: Atom(Int(3)),
                    indices: 18..=18
                },
                Token {
                    data: RightParen,
                    indices: 19..=19
                }
            ])
        );
    }*/
}
