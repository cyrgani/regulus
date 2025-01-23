use std::{error, fmt, result};

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
    Unimplemented,
}

#[derive(Debug)]
pub struct Exception {
    pub msg: String,
    pub error: Error,
}

impl Exception {
    #[expect(clippy::needless_pass_by_value, reason = "unhelpful warning")]
    pub fn new(msg: impl ToString, error: Error) -> Self {
        Self {
            msg: msg.to_string(),
            error,
        }
    }

    pub fn new_err<T>(msg: impl ToString, error: Error) -> Result<T> {
        Err(Self::new(msg, error))
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error, self.msg)
    }
}

impl error::Error for Exception {}

/// A shorthand alias for `Result<T, Exception>`.
pub type Result<T> = result::Result<T, Exception>;
