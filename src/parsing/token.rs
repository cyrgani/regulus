use crate::atom::Atom;
use crate::exception::{Error, Exception, ProgResult};
use std::cmp::Ordering;
use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub data: TokenData,
    pub span: Span,
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
    // TODO add this field
    // pub file: PathBuf,
    /// The start index of the span, inclusive.
    pub start: usize,
    /// The end index of the span, inclusive.
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.line == other.line {
            self.column.cmp(&other.column)
        } else {
            self.line.cmp(&other.line)
        }
    }
}

impl Position {
    pub const START: Self = Self { line: 1, column: 1 };
}

/// Takes characters from the stream until `target` is reached.
/// Returns all characters before `target` and the index of `target`.
fn take_until(chars: impl Iterator<Item = (usize, char)>, target: char) -> (usize, String) {
    let mut result = String::new();
    for (pos, c) in chars {
        if c == target {
            return (pos, result);
        }
        result.push(c);
    }
    // TODO: fix this position value
    (usize::MAX, result)
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut current = String::new();

    let mut chars = code.chars().enumerate();
    let mut add_token = |data, start, end| {
        tokens.push(Token {
            span: Span { start, end },
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
                        match Atom::try_from(current.as_str()) {
                            Ok(value) => TokenData::Atom(value),
                            Err(()) => TokenData::Name(current.clone()),
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
                let (end_pos, body) = take_until(chars.by_ref(), '"');
                add_token(TokenData::Atom(Atom::String(body)), char_idx, end_pos);
            }
            ' ' | '\n' | '\t' => (),
            '#' => {
                let (end_pos, body) = take_until(chars.by_ref(), '\n');
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

    tokens
}

pub fn validate_tokens(tokens: &[Token]) -> ProgResult<()> {
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

/// Returns all characters of the text that the given `Span` encloses.
/// Returns an empty string if the span is invalid (end before start or out of bounds).
pub fn extract(text: &str, span: Span) -> Option<String> {
    if span.start > span.end {
        return None;
    }
    let mut extracted = String::new();
    let mut extracting = false;
    for (pos, c) in text.chars().enumerate() {
        if pos == span.start {
            extracting = true;
        }
        if extracting {
            extracted.push(c);
        }
        if pos == span.end {
            if extracting {
                return Some(extracted);
            }
            return None;
        }
    }

    None
}

pub fn index_to_position(text: &str, idx: usize) -> Position {
    CharPositions::new(text).nth(idx).unwrap().0
}

pub struct CharPositions<'a> {
    text: Chars<'a>,
    pos: Position,
}

impl Iterator for CharPositions<'_> {
    type Item = (Position, char);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.text.next()?;
        let pos = self.pos;
        if next == '\n' {
            self.pos.line += 1;
            self.pos.column = 1;
        } else {
            self.pos.column += 1;
        }
        Some((pos, next))
    }
}

impl<'a> CharPositions<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text: text.chars(),
            pos: Position { line: 1, column: 1 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn p(line: usize, column: usize) -> Position {
        Position { line, column }
    }

    const fn pc(line: usize, column: usize, ch: char) -> (Position, char) {
        (p(line, column), ch)
    }

    #[test]
    fn span_iter() {
        let t = "abc\nde\nf\n";
        let i = CharPositions::new(t);
        assert_eq!(
            i.collect::<Vec<_>>(),
            [
                pc(1, 1, 'a'),
                pc(1, 2, 'b'),
                pc(1, 3, 'c'),
                pc(1, 4, '\n'),
                pc(2, 1, 'd'),
                pc(2, 2, 'e'),
                pc(2, 3, '\n'),
                pc(3, 1, 'f'),
                pc(3, 2, '\n'),
            ]
        );
    }

    const fn sp(start: usize, end: usize) -> Span {
        Span { start, end }
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
    fn pos_order() {
        assert!(p(2, 4) < p(2, 5));
        assert!(p(1, 3) > p(1, 1));
        assert!(p(1, 4) == p(1, 4));
        assert!(p(2, 1) > p(1, 10));
    }
}
