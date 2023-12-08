use std::fmt;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i32),
    Bool(bool),
    Null,
    List(Vec<Atom>),
    String(String),
    Function(Function),
}

impl TryFrom<&str> for Atom {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(int) = value.parse::<i32>() {
            Ok(Self::Int(int))
        } else {
            Ok(match value {
                "true" => Self::Bool(true),
                "false" => Self::Bool(false),
                "null" => Self::Null,
                _ => return Err(()),
            })
        }
    }
}

impl Atom {
    pub fn int(&self) -> ProgResult<i32> {
        match self {
            Self::Int(v) => Ok(*v),
            _ => Err(Exception {
                msg: format!("{self:?} is not a Int!"),
                error: Error::Type,
            }),
        }
    }

    pub fn bool(&self) -> ProgResult<bool> {
        match self {
            Self::Bool(v) => Ok(*v),
            _ => Err(Exception {
                msg: format!("{self:?} is not a Bool!"),
                error: Error::Type,
            }),
        }
    }

    pub fn list(&self) -> ProgResult<Vec<Self>> {
        match self {
            Self::List(v) => Ok(v.clone()),
            _ => Err(Exception {
                msg: format!("{self:?} is not a List!"),
                error: Error::Type,
            }),
        }
    }

    pub fn string(&self) -> ProgResult<String> {
        match self {
            Self::String(v) => Ok(v.clone()),
            _ => Err(Exception {
                msg: format!("{self:?} is not a String!"),
                error: Error::Type,
            }),
        }
    }

    pub fn function(&self) -> ProgResult<Function> {
        match self {
            Self::Function(v) => Ok(v.clone()),
            _ => Err(Exception {
                msg: format!("{self:?} is not a Function!"),
                error: Error::Type,
            }),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(val) => val.to_string(),
                Self::Function(val) => format!("{}()", val.name),
                Self::Int(val) => val.to_string(),
                Self::List(val) => format!(
                    "[{}]",
                    val.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Self::Null => "null".to_string(),
                Self::String(val) => val.clone(),
            }
        )
    }
}
