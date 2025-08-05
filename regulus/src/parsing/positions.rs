use std::cmp::Ordering;
use std::fmt::Display;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::Chars;

/// A region of source code.
/// Both start and end are inclusive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// The file path this span points to.
    pub file: Rc<PathBuf>,
    /// The start position of the span, inclusive.
    pub start: Position,
    /// The end position of the span, inclusive.
    pub end: Position,
}

impl Span {
    pub const fn new(start: Position, end: Position, file: Rc<PathBuf>) -> Self {
        Self { file, start, end }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            if self.file.to_str() == Some("") {
                "<file>".to_string()
            } else {
                self.file.display().to_string()
            },
            self.start.line - 1,
            self.start.column,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The line, starting at 1.
    pub line: u32,
    /// The column, starting at 1.
    pub column: u32,
}

impl Position {
    pub const ONE: Self = Self { line: 1, column: 1 };
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
    pub(crate) const fn one_back(self) -> Self {
        Self {
            column: self
                .column
                .checked_sub(1)
                .expect("one_back should not be used before \\n"),
            line: self.line,
        }
    }
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

    #[test]
    fn sizes_of_types() {
        assert_eq!(size_of::<Position>(), 8);
        assert_eq!(size_of::<Span>(), 24);
    }

    const fn p(line: u32, column: u32) -> Position {
        Position { line, column }
    }

    const fn pc(line: u32, column: u32, ch: char) -> (Position, char) {
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

    #[test]
    fn pos_order() {
        assert!(p(2, 4) < p(2, 5));
        assert!(p(1, 3) > p(1, 1));
        assert!(p(1, 4) == p(1, 4));
        assert!(p(2, 1) > p(1, 10));
    }
}
