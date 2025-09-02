use crate::parsing::Span;
use std::{error, fmt, result};

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

#[derive(Debug, Clone)]
pub struct Exception {
    pub msg: String,
    pub error: String,
    // todo: this will eventually stop being optional
    pub backtrace: Option<Vec<Span>>,
}

impl Exception {
    /// Constructs an exception with the given error name and message,
    /// but without any span or backtrace.
    /// Using this method is discouraged; ideally it will be removed in the future.
    pub fn unspanned(error: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            error: error.into(),
            backtrace: None,
        }
    }

    /// Constructs an exception with the given error name and message,
    /// using the given span as the only backtrace entry.
    ///
    /// If you have a [`State`](crate::prelude::State) available,
    /// consider using [`State::raise`](crate::prelude::State::raise) instead.
    pub fn spanned(error: impl Into<String>, msg: impl Into<String>, span: &Span) -> Self {
        Self {
            msg: msg.into(),
            error: error.into(),
            backtrace: Some(vec![span.clone()]),
        }
    }

    /// Constructs an exception with the given error name, message and backtrace.
    ///
    /// If you have a [`State`](crate::prelude::State) available,
    /// consider using [`State::raise`](crate::prelude::State::raise) instead.
    pub fn with_trace(
        error: impl Into<String>,
        msg: impl Into<String>,
        backtrace: &[Span],
    ) -> Self {
        Self {
            msg: msg.into(),
            error: error.into(),
            backtrace: Some(backtrace.to_vec()),
        }
    }
}

/// Creates an exception wrapped in an `Err`.
///
/// The first argument is the current `State`, which is used to add a backtrace to the call.
/// The second argument is the kind of the exception, the third the message or format string.
/// Any further arguments are passed into the `format!` string.
#[macro_export]
macro_rules! raise {
    ($state: expr, $kind: expr, $string: literal) => {
        $crate::raise!($state, $kind, $string,)
    };
    ($state: expr, $kind: expr, $msg: expr) => {
        $crate::raise!($state, $kind, "{}", $msg)
    };
    ($state: expr, $kind: expr, $string: literal, $($fmt_args: expr),*) => {
        return Err(Exception::with_trace($kind, format!($string, $($fmt_args),*), &$state.backtrace))
    };
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}Error: {}", self.error, self.msg)?;
        if let Some(backtrace) = self.backtrace.as_ref() {
            if backtrace.len() == 1 {
                // in the case of a syntax error, the backtrace is just the error location
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
