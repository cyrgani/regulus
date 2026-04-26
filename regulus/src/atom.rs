use crate::exception::{OverflowError};
use crate::list::List;
use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i64),
    Bool(bool),
    Char(char),
    Null,
    List(List),
    Function(Function),
    Object(Object),
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs.partial_cmp(rhs),
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs.partial_cmp(rhs),
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl Atom {
    pub(crate) fn int_from_rust_int<T>(val: T, state: &State) -> Result<Self>
    where
        i64: TryFrom<T>,
        <i64 as TryFrom<T>>::Error: Display,
    {
        match i64::try_from(val) {
            Ok(int) => Ok(Self::Int(int)),
            Err(e) => raise!(state, OverflowError, "invalid integer: {e}"),
        }
    }

    pub const INT_TY_ID: i64 = 0;
    pub const BOOL_TY_ID: i64 = 1;
    pub const CHAR_TY_ID: i64 = 2;
    pub const NULL_TY_ID: i64 = 3;
    pub const LIST_TY_ID: i64 = 4;
    pub const FUNCTION_TY_ID: i64 = 5;
    pub const MIN_OBJECT_TY_ID: i64 = 6;

    pub const fn ty_id(&self) -> i64 {
        match self {
            Self::Int(_) => Self::INT_TY_ID,
            Self::Bool(_) => Self::BOOL_TY_ID,
            Self::Char(_) => Self::CHAR_TY_ID,
            Self::Null => Self::NULL_TY_ID,
            Self::List(_) => Self::LIST_TY_ID,
            Self::Function(_) => Self::FUNCTION_TY_ID,
            Self::Object(o) => o.ty_id,
        }
    }

    /// Constructs a new `List`.
    pub fn new_list(v: Vec<Self>) -> Self {
        Self::List(List::new(v))
    }

    /// Constructs a new string, represented as a `List` of `Char`s.
    pub fn new_string(s: &str) -> Self {
        Self::new_list(s.chars().map(Self::Char).collect())
    }

    /// If this is a `List` in which all elements are `Char`s,
    /// this concatenates them into a `String` and returns it.
    /// Otherwise, it returns `None`.
    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::List(l) => l
                .iter()
                .map(|el| match el {
                    Self::Char(c) => Some(*c),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }

    /// Contructs an object with the type id `i64::MAX` directly.
    /// Useful for (singleton) objects added from outside the language.
    pub const fn new_object(data: HashMap<String, Self>) -> Self {
        Self::Object(Object::new(data, i64::MAX))
    }

    pub fn stringify(&self) -> String {
        if let Some(s) = self.as_string() {
            return format!("\"{s}\"");
        }
        match self {
            Self::Char(c) => format!("'{c}'"),
            _ => self.to_string(),
        }
    }
}

macro_rules! atom_try_as_variant_methods {
    ($($method_name: ident: $variant:ident -> $ty:ty;)*) => {
        impl Atom {
            $(
                pub fn $method_name(&self) -> Option<$ty> {
                    match self {
                        Self::$variant(v) => Some(v.clone()),
                        _ => None,
                    }
                }
            )*
        }
    };
}

// method name: atom variant name -> rust type;
atom_try_as_variant_methods! {
    int: Int -> i64;
    bool: Bool -> bool;
    char: Char -> char;
    list: List -> List;
    function: Function -> Function;
    object: Object -> Object;
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_string() {
            return write!(f, "{s}");
        }
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
            Self::Char(val) => write!(f, "{val}"),
            Self::List(val) => write!(
                f,
                "[{}]",
                val.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Null => write!(f, "null"),
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

#[test]
fn type_id_matches() {
    macro_rules! t {
        ($n: ident) => {
            assert_eq!(
                run(format!("import(type_id), {}", stringify!($n)))
                    .unwrap()
                    .int()
                    .unwrap(),
                Atom::$n
            );
        };
    }
    t!(INT_TY_ID);
    t!(BOOL_TY_ID);
    t!(CHAR_TY_ID);
    t!(NULL_TY_ID);
    t!(LIST_TY_ID);
    t!(FUNCTION_TY_ID);
    t!(MIN_OBJECT_TY_ID);
}
