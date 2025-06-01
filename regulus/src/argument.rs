use crate::parsing::positions::Span;
use crate::prelude::*;
use std::borrow::Cow;
#[cfg(feature = "display_impls")]
use std::fmt;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Argument {
    FunctionCall(FunctionCall, Span),
    Atom(Atom, Span),
    Variable(String, Span),
}

impl Argument {
    pub fn eval<'a>(&'a self, state: &'a mut State) -> Result<Cow<'a, Atom>> {
        if state.exit_unwind_value.is_some() {
            return Ok(Cow::Owned(Atom::Null));
        }
        match self {
            Self::FunctionCall(call, _) => call.eval(state).map(Cow::Owned),
            Self::Atom(atom, _) => Ok(Cow::Borrowed(atom)),
            Self::Variable(var, _) => match state.storage.get(var) {
                Some(value) => Ok(Cow::Borrowed(value)),
                None => raise!(Error::Name, "No variable named `{var}` found!"),
            },
        }
    }

    /// Returns the identifier of this variable.
    /// If it is not a variable, it raises an exception with the given error message.
    pub fn variable(&self, error_msg: &str) -> Result<&String> {
        match self {
            Self::Variable(var, _) => Ok(var),
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
