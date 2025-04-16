use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::str::Chars;

/// A region of source code.
/// Both start and end are inclusive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    // TODO add this field or rather not?
    // pub file: PathBuf,
    /// The start position of the span, inclusive.
    pub start: Position,
    /// The end position of the span, inclusive.
    pub end: Position,
}

impl Span {
    pub fn from_indices(indices: RangeInclusive<usize>, code: &str) -> Self {
        let mut positions = CharPositions::new(code);
        let (start, _) = positions.nth(*indices.start()).unwrap();
        let (end, _) = positions.nth(*indices.end() - *indices.start()).unwrap();
        Self { start, end }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The line, starting at 1.
    pub line: usize,
    /// The column, starting at 1.
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

    #[test]
    fn pos_order() {
        assert!(p(2, 4) < p(2, 5));
        assert!(p(1, 3) > p(1, 1));
        assert!(p(1, 4) == p(1, 4));
        assert!(p(2, 1) > p(1, 10));
    }

    const fn sp(l1: usize, c1: usize, l2: usize, c2: usize) -> Span {
        Span {
            start: p(l1, c1),
            end: p(l2, c2),
        }
    }

    #[test]
    fn span_from_indices() {
        let s = "abc\nde\nf\n";
        assert_eq!(Span::from_indices(0..=2, s), sp(1, 1, 1, 4));
        assert_eq!(Span::from_indices(2..=6, s), sp(1, 3, 3, 1));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn span_from_indices_panic() {
        Span::from_indices(0..=1000, "abc\nde\nf\n");
    }
}
