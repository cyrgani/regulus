use std::fmt;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i64),
    Bool(bool),
    Null,
    List(Vec<Atom>),
    String(String),
    Function(Function),
}

impl TryFrom<&str> for Atom {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(int) = value.parse::<i64>() {
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

macro_rules! try_as_type {
    ($ty:ty, $variant:ident, $method_name:ident) => {
        pub fn $method_name(&self) -> ProgResult<$ty> {
            match self {
                Self::$variant(v) => Ok(v.clone()),
                _ => Exception::new_err(
                    format!("{self} is not a {}!", stringify!($variant)),
                    Error::Type,
                ),
            }
        }
    };
}

impl Atom {
    try_as_type! {i64, Int, int}
    try_as_type! {bool, Bool, bool}
    try_as_type! {Vec<Self>, List, list}
    try_as_type! {String, String, string}
    try_as_type! {Function, Function, function}
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(val) => val.to_string(),
                Self::Function(_) => "<function>".to_string(),
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
