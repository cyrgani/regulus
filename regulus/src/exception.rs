//! TODO: the future of exception handling:
//!  - exceptions have spans and tracebacks
//!  - `Error` will be removed
//!  - `catch(1)`'s functionality (exception -> string) will remain but might be renamed
use crate::parsing::positions::Span;
use std::{error, fmt, result};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
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
    DivideByZero,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Self::Other(s) = self {
            write!(f, "{s}Error")
        } else {
            write!(f, "{self:?}Error")
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Exception {
    pub msg: String,
    pub error: Error,
    // todo: this will eventually stop being optional
    pub backtrace: Option<Vec<Span>>,
}

impl Exception {
    pub fn new(msg: impl Into<String>, error: Error) -> Self {
        Self {
            msg: msg.into(),
            error,
            backtrace: None,
        }
    }

    // TODO: remove?
    /// If you hae a [`State`](crate::prelude::State) available,
    /// consider using [`State::raise`](crate::prelude::State::raise) instead.
    pub fn spanned(msg: impl Into<String>, error: Error, span: &Span) -> Self {
        Self {
            msg: msg.into(),
            error,
            backtrace: Some(vec![span.clone()]),
        }
    }

    /// If you hae a [`State`](crate::prelude::State) available,
    /// consider using [`State::raise`](crate::prelude::State::raise) instead.
    pub fn with_trace(error: Error, msg: impl Into<String>, backtrace: &[Span]) -> Self {
        Self {
            msg: msg.into(),
            error,
            backtrace: Some(backtrace.to_vec()),
        }
    }
}

/// Creates an exception wrapped in an `Err` and returns it from the current function or closure.
///
/// The first argument is the kind of the exception, the second the message or format string.
/// Any further arguments are passed into the `format!` string.
#[macro_export]
macro_rules! raise {
    ($($t: tt)*) => {
        return $crate::raise_noreturn!($($t)*)
    }
}

/// Creates an exception wrapped in an `Err`.
///
/// The first argument is the kind of the exception, the second the message or format string.
/// Any further arguments are passed into the `format!` string.
#[macro_export]
macro_rules! raise_noreturn {
    ($kind: expr, $string: literal) => {
        $crate::raise_noreturn!($kind, $string,)
    };
    ($kind: expr, $msg: expr) => {
        $crate::raise_noreturn!($kind, "{}", $msg)
    };
    ($kind: expr, $string: literal, $($fmt_args: expr),*) => {
        Err(Exception::new(format!($string, $($fmt_args),*), $kind))
    };
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.msg)?;
        if let Some(backtrace) = self.backtrace.as_ref() {
            // the first entry is the meaningless implicit `_` wrapper
            for span in backtrace.iter().skip(1).rev() {
                writeln!(f)?;
                write!(
                    f,
                    "at {}:{}:{}",
                    if span.file.to_str() == Some("") {
                        "<file>".to_string()
                    } else {
                        span.file.display().to_string()
                    },
                    span.start.line - 1,
                    span.start.column,
                )?;
            }
        }
        Ok(())
    }
}

impl error::Error for Exception {}

/// A shorthand alias for `Result<T, Exception>`.
pub type Result<T> = result::Result<T, Exception>;
