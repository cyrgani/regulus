/// TODO: the future of exception handling:
///  - exceptions have spans and tracebacks
///  - `Error` will be removed
///  - `catch(1)`'s functionality (exception -> string) will remain but might be renamed
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
}

/// Creates an exception wrapped in an `Err`.
/// The first argument is the kind of the exception, the second the message or format string.
/// Any further arguments are passed into the `format!` string.
#[macro_export]
macro_rules! raise {
    ($kind: expr, $string: literal) => {
        raise!($kind, $string,)
    };
    ($kind: expr, $string: literal, $($fmt_args: expr),*) => {
        Err(Exception::new(format!($string, $($fmt_args),*), $kind))
    };
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error, self.msg)
    }
}

impl error::Error for Exception {}

/// A shorthand alias for `Result<T, Exception>`.
pub type Result<T> = result::Result<T, Exception>;
