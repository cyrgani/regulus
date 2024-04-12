use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Argument {
    FunctionCall(FunctionCall),
    Atom(Atom),
    Variable(String),
}

impl Argument {
    pub fn eval(&self, storage: &mut Storage) -> ProgResult<Atom> {
        match self {
            Self::FunctionCall(call) => call.eval(storage),
            Self::Atom(atom) => Ok(atom.clone()),
            Self::Variable(var) => match storage.get(var) {
                Some(value) => Ok(value.clone()),
                None => {
                    Exception::new_err(format!("No variable named `{var}` found!"), Error::Name)
                }
            },
        }
    }
}

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
