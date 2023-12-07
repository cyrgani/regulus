use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Type,
    Overflow,
    Name,
    Syntax,
    Argument,
    Assign,
    Index,
    Io,
    Import,
    UserRaised,
    Assertion,
}

#[derive(Debug)]
pub struct Exception {
    pub msg: String,
    pub error: Error,
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error, self.msg)
    }
}

impl error::Error for Exception {}

/// A shorthand alias for `Result<T, Exception>`.
pub type ProgResult<T> = Result<T, Exception>;
