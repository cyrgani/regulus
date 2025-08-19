use crate::parsing::positions::Span;
use std::{error, fmt, result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

#[expect(non_upper_case_globals)]
mod errors {
    pub(crate) const TypeError: &str = "Type";
    pub(crate) const OverflowError: &str = "Overflow";
    pub(crate) const NameError: &str = "Name";
    pub(crate) const SyntaxError: &str = "Syntax";
    pub(crate) const ArgumentError: &str = "Argument";
    pub(crate) const IndexError: &str = "Index";
    pub(crate) const IoError: &str = "Io";
    pub(crate) const ImportError: &str = "Import";
    pub(crate) const DivideByZeroError: &str = "DivideByZero";
}

pub(crate) use errors::*;

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}Error", self.0)
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
    pub fn new(error: impl Into<Error>, msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            error: error.into(),
            backtrace: None,
        }
    }

    // TODO: remove?
    /// If you have a [`State`](crate::prelude::State) available,
    /// consider using [`State::raise`](crate::prelude::State::raise) instead.
    pub fn spanned(error: impl Into<Error>, msg: impl Into<String>, span: &Span) -> Self {
        Self {
            msg: msg.into(),
            error: error.into(),
            backtrace: Some(vec![span.clone()]),
        }
    }

    /// If you have a [`State`](crate::prelude::State) available,
    /// consider using [`State::raise`](crate::prelude::State::raise) instead.
    pub fn with_trace(error: impl Into<Error>, msg: impl Into<String>, backtrace: &[Span]) -> Self {
        Self {
            msg: msg.into(),
            error: error.into(),
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
        Err(Exception::new($kind, format!($string, $($fmt_args),*)))
    };
    ($state: expr, $kind: expr, $string: literal) => {
        $crate::raise_noreturn!($state, $kind, $string,)
    };
    ($state: expr, $kind: expr, $msg: expr) => {
        $crate::raise_noreturn!($state, $kind, "{}", $msg)
    };
    ($state: expr, $kind: expr, $string: literal, $($fmt_args: expr),*) => {
        Err(Exception::with_trace($kind, format!($string, $($fmt_args),*), &$state.backtrace))
    };
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.msg)?;
        if let Some(backtrace) = self.backtrace.as_ref() {
            if backtrace.len() == 1 {
                // in the case of a syntax error, the backtrace it just the error location
                write!(f, "\nat {}", backtrace[0])?;
            } else {
                // otherwise, the first entry is the meaningless implicit `_` wrapper
                for span in backtrace.iter().skip(1).rev() {
                    write!(f, "\nat {span}")?;
                }
            }
        }
        Ok(())
    }
}

impl error::Error for Exception {}

/// A shorthand alias for `Result<T, Exception>`.
pub type Result<T> = result::Result<T, Exception>;
