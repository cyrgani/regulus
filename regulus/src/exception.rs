//! TODO: the future of exception handling:
//!  - exceptions have spans and tracebacks
//!  - `Error` will be removed
//!  - `catch(1)`'s functionality (exception -> string) will remain but might be renamed
use crate::parsing::positions::{ExpandedSpan, Span};
use crate::prelude::State;
use std::{error, fmt, result};
use std::path::PathBuf;

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
    // todo: this will eventually stop being optional
    pub origin: Option<Span>,
}

impl Exception {
    pub fn new(msg: impl Into<String>, error: Error) -> Self {
        Self {
            msg: msg.into(),
            error,
            origin: None,
        }
    }

    pub fn spanned(msg: impl Into<String>, error: Error, span: Span) -> Self {
        Self {
            msg: msg.into(),
            error,
            origin: Some(span),
        }
    }

    pub fn display(&self, state: &State) -> impl error::Error {
        ExceptionDisplay {
            msg: &self.msg,
            error: &self.error,
            origin: self.origin.map(|span| span.expand(state)),
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

#[derive(Debug)]
struct ExceptionDisplay<'a> {
    msg: &'a String,
    error: &'a Error,
    // todo: will stop being optional soon
    origin: Option<ExpandedSpan>,
}

impl fmt::Display for ExceptionDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(origin) = self.origin.as_ref() {
            write!(
                f,
                "{}:{}:{}: {}: {}",
                // TODO: this check might be temporary
                if origin.file == PathBuf::new() {
                    "<file>".to_string()
                } else {
                    origin.file.display().to_string()
                },
                origin.start.line - 1,
                origin.start.column,
                self.error,
                self.msg
            )
        } else {
            write!(f, "{}: {}", self.error, self.msg)
        }
    }
}

impl error::Error for ExceptionDisplay<'_> {}

/// A shorthand alias for `Result<T, Exception>`.
pub type Result<T> = result::Result<T, Exception>;
