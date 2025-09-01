use crate::exception::{OverflowError, SyntaxError, TypeError};
use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::num::IntErrorKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i64),
    Bool(bool),
    Null,
    List(Vec<Atom>),
    String(String),
    Function(Function),
    Object(Object),
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
                        SyntaxError,
                        "integer {value} cannot be parsed as an integer due to overflow",
                    ),
                    _ => Ok(None),
                },
            },
        }
    }

    // TODO: make this public?
    pub(crate) fn int_from_rust_int<T>(val: T) -> Result<Self>
    where
        i64: TryFrom<T>,
        <i64 as TryFrom<T>>::Error: Display,
    {
        match i64::try_from(val) {
            Ok(int) => Ok(Self::Int(int)),
            Err(e) => raise!(OverflowError, "invalid integer: {e}"),
        }
    }

    pub const INT_TY_ID: i64 = 0;
    pub const BOOL_TY_ID: i64 = 1;
    pub const NULL_TY_ID: i64 = 2;
    pub const LIST_TY_ID: i64 = 3;
    pub const STRING_TY_ID: i64 = 4;
    pub const FUNCTION_TY_ID: i64 = 5;
    pub const MIN_OBJECT_TY_ID: i64 = 6;

    pub const fn ty_id(&self) -> i64 {
        match self {
            Self::Int(_) => Self::INT_TY_ID,
            Self::Bool(_) => Self::BOOL_TY_ID,
            Self::Null => Self::NULL_TY_ID,
            Self::List(_) => Self::LIST_TY_ID,
            Self::String(_) => Self::STRING_TY_ID,
            Self::Function(_) => Self::FUNCTION_TY_ID,
            Self::Object(o) => o.ty_id,
        }
    }

    /// Contructs an object with the type id `i64::MAX` directly.
    /// Useful for (singleton) objects added from outside the language.
    pub const fn new_object(data: HashMap<String, Self>) -> Self {
        Self::Object(Object::new(data, i64::MAX))
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
                            TypeError,
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
    object -> Object: Object;
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(val) => write!(f, "{val}"),
            Self::Function(func) => write!(
                f,
                "<function>({})",
                match func.argc() {
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
            Self::Object(obj) => {
                write!(f, "{{")?;
                let mut ordered = obj.data.iter().collect::<Vec<_>>();
                ordered.sort_by_key(|(field, _)| *field);
                for (idx, (key, val)) in ordered.iter().enumerate() {
                    write!(f, "{key}: {val}")?;
                    if idx != obj.data.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub data: HashMap<String, Atom>,
    pub ty_id: i64,
}

impl Object {
    pub const fn new(data: HashMap<String, Atom>, ty_id: i64) -> Self {
        Self { data, ty_id }
    }
}
