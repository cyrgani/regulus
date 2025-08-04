use crate::state::State;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::Chars;

// TODO: yet another idea: we could use only `ExpandedSpan` and store the path in an `Rc` to make clones cheap.
//  then, ExpandedSpan is 24 bytes. Span would be removed and ExpandedSpan renamed to Span.
//  the new Span does not require a State to expand, since it is already expanded.
//  the new Span is not Copy though and is larger than the current span (12 bytes).

/// A memory-efficient version of [`ExpandedSpan`].
/// Using a [`State`](crate::prelude::State), this can be converted into an [`ExpandedSpan`]
/// for display purposes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// inclusive
    pub start: u32,
    /// inclusive
    pub end: u32,
    pub file_path: Rc<PathBuf>,
}

impl Span {
    pub const fn new(start: u32, end: u32, file_path: Rc<PathBuf>) -> Self {
        Self {
            start,
            end,
            file_path,
        }
    }

    pub const fn len(&self) -> u32 {
        self.end + 1 - self.start
    }

    pub fn expand(&self, state: &State) -> ExpandedSpan {
        let path = self.file_path.clone();
        ExpandedSpan::from_span(self, state.code(), path.as_ref())
    }
}

/// A region of source code.
/// Both start and end are inclusive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandedSpan {
    /// The file path this span points to.
    pub file: PathBuf,
    /// The start position of the span, inclusive.
    pub start: Position,
    /// The end position of the span, inclusive.
    pub end: Position,
}

impl ExpandedSpan {
    /// TODO: This might be removed in favor of [`Span::expand`].
    pub fn from_span(span: &Span, code: &str, file: impl AsRef<Path>) -> Self {
        // TODO: horribly inefficient to redo the iteration for each span,
        //  better: just do the iteration once, collect and then pass the slice to this function
        dbg!(&span, code, file.as_ref());
        let mut positions = CharPositions::new(code);
        let (start, _) = positions.nth(span.start as usize).unwrap();
        let file = file.as_ref().to_path_buf();
        if span.start == span.end {
            return Self {
                start,
                end: start,
                file,
            };
        }
        let (end, _) = positions.nth((span.end - span.start) as usize).unwrap();
        Self { file, start, end }
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
    use std::rc::Rc;

    #[test]
    fn sizes_of_types() {
        assert_eq!(size_of::<Position>(), 8);
        assert_eq!(size_of::<Rc<PathBuf>>(), 8);
        assert_eq!(size_of::<Span>(), 16);
        assert_eq!(size_of::<ExpandedSpan>(), 40);
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

    fn sp(l1: u32, c1: u32, l2: u32, c2: u32) -> ExpandedSpan {
        ExpandedSpan {
            file: PathBuf::new(),
            start: p(l1, c1),
            end: p(l2, c2),
        }
    }

    fn base_sp(start: u32, end: u32) -> Span {
        Span {
            start,
            end,
            file_path: Rc::new(PathBuf::new()),
        }
    }

    #[test]
    fn span_from_indices() {
        let s = "abc\nde\nf\n";
        assert_eq!(
            ExpandedSpan::from_span(&base_sp(0, 2), s, ""),
            sp(1, 1, 1, 4)
        );
        assert_eq!(
            ExpandedSpan::from_span(&base_sp(2, 6), s, ""),
            sp(1, 3, 3, 1)
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn span_from_indices_panic() {
        ExpandedSpan::from_span(&base_sp(0, 1000), "abc\nde\nf\n", "");
    }
}
