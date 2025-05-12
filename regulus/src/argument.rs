use crate::prelude::*;
use std::borrow::Cow;
#[cfg(feature = "display_impls")]
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Argument {
    pub data: ArgumentData,
    pub span_indices: RangeInclusive<usize>,
}

#[derive(Debug, Clone)]
pub enum ArgumentData {
    FunctionCall(FunctionCall),
    Atom(Atom),
    Variable(String),
}

impl Argument {
    pub fn eval<'a>(&'a self, state: &'a mut State) -> Result<Cow<'a, Atom>> {
        if state.exit_unwind_value.is_some() {
            return Ok(Cow::Owned(Atom::Null));
        }
        match &self.data {
            ArgumentData::FunctionCall(call) => call.eval(state).map(Cow::Owned),
            ArgumentData::Atom(atom) => Ok(Cow::Borrowed(atom)),
            ArgumentData::Variable(var) => match state.storage.get(var) {
                Some(value) => Ok(Cow::Borrowed(value)),
                None => raise!(Error::Name, "No variable named `{var}` found!"),
            },
        }
    }

    /// Returns the identifier of this variable.
    /// If it is not a variable, it raises an exception with the given error message.
    pub fn variable(&self, error_msg: &str) -> Result<&String> {
        match &self.data {
            ArgumentData::Variable(var) => Ok(var),
            _ => raise!(Error::Argument, error_msg),
        }
    }
}

#[cfg(feature = "display_impls")]
impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            ArgumentData::Atom(atom) => write!(f, "{atom}"),
            ArgumentData::FunctionCall(call) => write!(f, "{call}"),
            ArgumentData::Variable(name) => write!(f, "{name}"),
        }
    }
}
