use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::num::IntErrorKind;

#[derive(Debug, PartialEq)]
pub enum Atom {
    Int(i64),
    Bool(bool),
    Null,
    List(Vec<Atom>),
    String(String),
    Function(Function),
    Object(HashMap<String, Atom>),
}


impl Clone for Atom {
    fn clone(&self) -> Self {
        crate::clone_investigate(self);
        match self {
            Self::Int(i) => Self::Int(*i),
            Self::Bool(b) => Self::Bool(*b),
            Self::Null => Self::Null,
            Self::List(l) => Self::List(l.clone()),
            Self::String(s) => Self::String(s.clone()),
            Self::Function(f) => Self::Function(f.clone()),
            Self::Object(o) => Self::Object(o.clone()),
        }
    }
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
    object -> HashMap<String, Atom>: Object;
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(val) => write!(f, "{val}"),
            Self::Function(func) => write!(
                f,
                "<function>({})",
                match func.argc {
                    Some(argc) => argc.to_string(),
                    None => "_".to_string(),
                }
            ),
            Self::Int(val) => write!(f, "{val}"),
            Self::List(val) => write!(
                f,
                "[{}]",
                val.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Null => write!(f, "null"),
            Self::String(val) => write!(f, "{val}"),
            // todo: investigate the proper format
            Self::Object(obj) => write!(f, "{obj:?}"),
        }
    }
}
