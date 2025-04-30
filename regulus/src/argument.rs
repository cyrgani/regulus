use crate::prelude::*;
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
    pub fn eval(&self, state: &mut State) -> Result<Atom> {
        if state.exit_unwind_value.is_some() {
            return Ok(Atom::Null);
        }
        match &self.data {
            ArgumentData::FunctionCall(call) => call.eval(state),
            ArgumentData::Atom(atom) => Ok(atom.clone()),
            ArgumentData::Variable(var) => match state.storage.get(var) {
                Some(value) => Ok(value.clone()),
                None => raise!(Error::Name, "No variable named `{var}` found!"),
            },
        }
    }

    /// Returns the identifier of this variable.
    /// If it is not a variable, it raises an exception with the given error message.
    pub fn variable(&self, error_msg: &str) -> Result<&String> {
        match &self.data {
            ArgumentData::Variable(var) => Ok(var),
            _ => raise!(Error::Argument, "{error_msg}"),
        }
    }

    /// If this is a function call, it is returned.
    /// Otherwise, it raises an exception with the given error message.
    pub fn function_call(&self, error_msg: &str) -> Result<FunctionCall> {
        match &self.data {
            ArgumentData::FunctionCall(call) => Ok(call.clone()),
            _ => raise!(Error::Argument, "{error_msg}"),
        }
    }
}

#[cfg(feature = "display_impls")]
impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.data {
                ArgumentData::Atom(atom) => atom.to_string(),
                ArgumentData::FunctionCall(call) => call.to_string(),
                ArgumentData::Variable(name) => name.to_string(),
            }
        )
    }
}
