use crate::prelude::*;
use std::fmt;
use std::num::IntErrorKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i64),
    Bool(bool),
    Null,
    List(Vec<Atom>),
    String(String),
    Function(Function),
}

impl Atom {
    pub fn try_from_str(value: &str) -> Result<Option<Self>> {
        match value {
            "true" => Ok(Some(Self::Bool(true))),
            "false" => Ok(Some(Self::Bool(false))),
            "null" => Ok(Some(Self::Null)),
            _ => match value.parse::<i64>() {
                Ok(int) => Ok(Some(Self::Int(int))),
                Err(err) => match err.kind() {
                    IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => raise!(
                        Error::Syntax,
                        "integer {value} cannot be parsed as an integer due to overflow",
                    ),
                    _ => Ok(None),
                },
            },
        }
    }
}

macro_rules! atom_try_as_variant_methods {
    ($($method_name: ident -> $ty:ty: $variant:ident;)*) => {
        impl Atom {
            $(
                pub fn $method_name(&self) -> Result<$ty> {
                    match self {
                        Self::$variant(v) => Ok(v.clone()),
                        _ => raise!(
                            Error::Type,
                            "{self} is not a {}!", stringify!($variant)
                        ),
                    }
                }
            )*
        }
    };
}

// method name, rust type, atom variant name
atom_try_as_variant_methods! {
    int -> i64: Int;
    bool -> bool: Bool;
    list -> Vec<Self>: List;
    string -> String: String;
    function -> Function: Function;
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(val) => val.to_string(),
                Self::Function(f) => format!(
                    "<function>({})",
                    match f.argc {
                        Some(argc) => argc.to_string(),
                        None => "...".to_string(),
                    }
                ),
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
