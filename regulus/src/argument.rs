use crate::parsing::positions::Span;
use crate::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Argument {
    FunctionCall(FunctionCall, Span),
    Atom(Atom, Span),
    Variable(String, Span),
}

impl Argument {
    pub fn eval<'a>(&'a self, state: &'a mut State) -> Result<Cow<'a, Atom>> {
        state.current_span = self.span();
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

    /// Returns an approximation of the source code of this argument.
    pub fn stringify(&self) -> String {
        match self {
            Self::Atom(atom, _) => atom.to_string(),
            Self::FunctionCall(call, _) => call.stringify(),
            Self::Variable(name, _) => name.to_string(),
        }
    }
    
    // TODO: consider making this public
    pub(crate) const fn span(&self) -> Span {
        match self { 
            Self::Atom(_, s) | Self::FunctionCall(_, s) | Self::Variable(_, s) => *s,
        }
    }
}
