use std::{error, fmt};

#[derive(Debug)]
pub enum ErrorClass {
    TypeError,
    OverflowError,
    NameError,
    SyntaxError,
    ArgumentError,
    AssignError,
    IndexError,
    IoError,
    ImportError,
    UserRaisedError,
    AssertionError,
}

#[derive(Debug)]
pub struct ProgError {
    pub msg: String,
    pub class: ErrorClass,
}

impl fmt::Display for ProgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.class, self.msg)
    }
}

impl error::Error for ProgError {}

/// A shorthand alias for `Result<T, ProgError>`.
pub type ProgResult<T> = Result<T, ProgError>;
