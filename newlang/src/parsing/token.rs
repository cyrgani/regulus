use crate::atom::Atom;
use crate::exception::{Error, Exception, Result};
use std::ops::RangeInclusive;
use std::result;

#[derive(Debug)]
pub struct Token {
    pub data: TokenData,
    pub indices: RangeInclusive<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    Function(String),
    LeftParen,
    Comma,
    RightParen,
    Atom(Atom),
    Name(String),
    Comment(String),
}

/// A region of source code.
/// Both start and end are inclusive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    // TODO add this field or rather not?
    // pub file: PathBuf,
    /// The start index of the span, inclusive.
    pub start: usize,
    /// The end index of the span, inclusive.
    pub end: usize,
}

impl From<RangeInclusive<usize>> for Span {
    fn from(range: RangeInclusive<usize>) -> Self {
        Self {
            start: *range.start(),
            end: *range.end(),
        }
    }
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
            ')' | ',' => {
                if !current.is_empty() {
                    add_token(
                        match Atom::try_from_str(current.as_str()) {
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
                        _ => unreachable!(),
                    },
                    char_idx,
                    char_idx,
                );
            }
            '"' => {
                let Ok((end_pos, body)) = take_until(chars.by_ref(), '"') else {
                    return Exception::new_err("unclosed string literal", Error::Syntax);
                };
                add_token(TokenData::Atom(Atom::String(body)), char_idx, end_pos);
            }
            ' ' | '\n' | '\t' => (),
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
            return Exception::new_err(
                format!("More ')' ({right_parens}) than '(' ({left_parens}) at some time!"),
                Error::Syntax,
            );
        }
    }

    if left_parens != right_parens {
        return Exception::new_err(
            format!("Nonequal amount of '(' and ')': {left_parens} vs. {right_parens}"),
            Error::Syntax,
        );
    }

    Ok(())
}

/// Returns all characters of the text that the given indices enclose.
/// Returns an empty string if the indices are invalid (end before start or out of bounds).
pub fn extract(text: &str, indices: RangeInclusive<usize>) -> Option<String> {
    if indices.start() > indices.end() {
        return None;
    }
    let mut extracted = String::new();
    let mut extracting = false;
    for (pos, c) in text.chars().enumerate() {
        if pos == *indices.start() {
            extracting = true;
        }
        if extracting {
            extracted.push(c);
        }
        if pos == *indices.end() {
            if extracting {
                return Some(extracted);
            }
            return None;
        }
    }

    None
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
}
