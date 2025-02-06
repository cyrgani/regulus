use crate::prelude::*;
#[cfg(feature = "display_impls")]
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Argument {
    pub data: ArgumentData,
    pub indices: RangeInclusive<usize>,
}

#[derive(Debug, Clone)]
pub enum ArgumentData {
    FunctionCall(FunctionCall),
    Atom(Atom),
    Variable(String),
}

impl Argument {
    pub fn eval(&self, state: &mut State) -> Result<Atom> {
        match &self.data {
            ArgumentData::FunctionCall(call) => call.eval(state),
            ArgumentData::Atom(atom) => Ok(atom.clone()),
            ArgumentData::Variable(var) => match state.storage.get(var) {
                Some(value) => Ok(value.clone()),
                None => {
                    raise!(Error::Name, "No variable named `{var}` found!")
                }
            },
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
