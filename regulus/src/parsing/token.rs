use crate::atom::Atom;
use crate::exception::{Error, Exception, Result};
use crate::prelude::Argument;
use crate::prelude::ArgumentData;
use crate::raise;
use std::ops::RangeInclusive;
use std::result;

/// A token of source code with location information.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Token {
    /// The actual token.
    pub data: TokenData,
    /// The start and end of the character range this token was created from.
    pub indices: RangeInclusive<usize>,
}

impl Token {
    pub(crate) fn to_atom(&self) -> Option<Argument> {
        if let TokenData::Atom(atom) = &self.data {
            Some(Argument {
                data: ArgumentData::Atom(atom.clone()),
                span_indices: self.indices.clone(),
            })
        } else {
            None
        }
    }

    pub(crate) fn to_name(&self) -> Option<Argument> {
        if let TokenData::Name(name) | TokenData::Function(name) = &self.data {
            Some(Argument {
                data: ArgumentData::Variable(name.to_string()),
                span_indices: self.indices.clone(),
            })
        } else {
            None
        }
    }

    pub(crate) fn name(&self) -> Option<String> {
        if let TokenData::Name(name) | TokenData::Function(name) = &self.data {
            Some(name.to_string())
        } else {
            None
        }
    }

    pub(crate) const fn is_left_paren(&self) -> bool {
        matches!(self.data, TokenData::LeftParen)
    }

    pub(crate) const fn is_right_paren(&self) -> bool {
        matches!(self.data, TokenData::RightParen)
    }

    pub(crate) const fn is_comma(&self) -> bool {
        matches!(self.data, TokenData::Comma)
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TokenData {
    #[deprecated]
    Function(String),
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
    chars: impl Iterator<Item = (usize, char)>,
    target: char,
) -> result::Result<(usize, String), String> {
    let mut result = String::new();
    for (pos, c) in chars {
        if c == target {
            return Ok((pos, result));
        }
        result.push(c);
    }
    Err(result)
}

pub fn tokenize(code: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];

    let mut current = String::new();

    let mut chars = code.chars().enumerate();
    let mut add_token = |data, start, end| {
        tokens.push(Token {
            indices: start..=end,
            data,
        });
    };

    let mut current_start_idx = None;

    while let Some((char_idx, c)) = chars.next() {
        match c {
            '(' => {
                if !current.is_empty() {
                    add_token(
                        TokenData::Function(current.clone()),
                        current_start_idx.unwrap(),
                        char_idx - 1,
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
                        char_idx - 1,
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
                    return raise!(Error::Syntax, "unclosed string literal");
                };
                add_token(TokenData::Atom(Atom::String(body)), char_idx, end_pos);
            }
            '#' => {
                let (end_pos, body) =
                    take_until(chars.by_ref(), '\n').unwrap_or_else(|body| (code.len() - 1, body));
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
            code.len(),
        );
    }

    Ok(tokens)
}

pub fn validate_tokens(tokens: &[Token]) -> Result<()> {
    let mut left_parens = 0;
    let mut right_parens = 0;

    for token in tokens {
        match token.data {
            TokenData::LeftParen => left_parens += 1,
            TokenData::RightParen => right_parens += 1,
            _ => (),
        }
        if right_parens > left_parens {
            return raise!(
                Error::Syntax,
                "More ')' ({right_parens}) than '(' ({left_parens}) at some time!"
            );
        }
    }

    if left_parens != right_parens {
        return raise!(
            Error::Syntax,
            "Nonequal amount of '(' and ')': {left_parens} vs. {right_parens}",
        );
    }

    Ok(())
}

/// Returns all characters of the text that the given indices enclose.
/// Returns `None` if the indices are invalid (end before start or out of bounds).
pub fn extract(text: &str, indices: RangeInclusive<usize>) -> Option<String> {
    if indices.start() > indices.end() || *indices.end() >= text.chars().count() {
        return None;
    }

    Some(
        text.chars()
            .skip(*indices.start())
            .take(indices.count())
            .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn sp(start: usize, end: usize) -> RangeInclusive<usize> {
        start..=end
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
        let tokens = tokenize(code).unwrap();

        let parts = tokens
            .into_iter()
            .map(|t| extract(code, t.indices).unwrap())
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

    use TokenData::*;

    #[expect(non_snake_case)]
    fn Function(name: &str) -> TokenData {
        TokenData::Function(name.to_string())
    }

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
                    data: Function("_"),
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
                    data: Function("_"),
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
                    data: Function("="),
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
                    data: Function("print"),
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
                    data: Function("print"),
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
    }
}
