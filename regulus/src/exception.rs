//! TODO: the future of exception handling:
//!  - exceptions have spans and tracebacks
//!  - `Error` will be removed
//!  - `catch(1)`'s functionality (exception -> string) will remain but might be renamed
use crate::parsing::positions::Position;
use std::{error, fmt, result};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}Error")
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Exception {
    pub msg: String,
    pub error: Error,
    pub origin: Option<Position>,
}

impl Exception {
    pub fn new(msg: impl Into<String>, error: Error) -> Self {
        Self {
            msg: msg.into(),
            error,
            origin: None,
        }
    }

    pub fn spanned(msg: impl Into<String>, error: Error, pos: Position) -> Self {
        Self {
            msg: msg.into(),
            error,
            origin: Some(pos),
        }
    }
}

/// Creates an exception wrapped in an `Err`.
/// The first argument is the kind of the exception, the second the message or format string.
/// Any further arguments are passed into the `format!` string.
#[macro_export]
macro_rules! raise {
    ($kind: expr, $string: literal) => {
        raise!($kind, $string,)
    };
    ($kind: expr, $msg: expr) => {
        raise!($kind, "{}", $msg)
    };
    ($kind: expr, $string: literal, $($fmt_args: expr),*) => {
        Err(Exception::new(format!($string, $($fmt_args),*), $kind))
    };
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(origin) = self.origin.as_ref() {
            write!(
                f,
                "{} at {}:{}: {}",
                self.error, origin.line, origin.column, self.msg
            )
        } else {
            write!(f, "{}: {}", self.error, self.msg)
        }
    }
}

impl error::Error for Exception {}

/// A shorthand alias for `Result<T, Exception>`.
pub type Result<T> = result::Result<T, Exception>;
