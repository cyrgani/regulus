use crate::prelude::*;
#[cfg(feature = "display_impls")]
use std::fmt;

#[derive(Debug, Clone)]
pub enum Argument {
    FunctionCall(FunctionCall),
    Atom(Atom),
    Variable(String),
}

impl Argument {
    pub fn eval(&self, state: &mut State) -> ProgResult<Atom> {
        match self {
            Self::FunctionCall(call) => call.eval(state),
            Self::Atom(atom) => Ok(atom.clone()),
            Self::Variable(var) => match state.storage.get(var) {
                Some(value) => Ok(value.clone()),
                None => {
                    Exception::new_err(format!("No variable named `{var}` found!"), Error::Name)
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
            match self {
                Self::Atom(atom) => atom.to_string(),
                Self::FunctionCall(call) => call.to_string(),
                Self::Variable(name) => name.to_string(),
            }
        )
    }
}
