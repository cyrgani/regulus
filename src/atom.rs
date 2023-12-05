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
            Ok(Atom::Int(int))
        } else {
            Ok(match value {
                "true" => Atom::Bool(true),
                "false" => Atom::Bool(false),
                "null" => Atom::Null,
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
                msg: format!("{:?} is not a Int!", self),
                error: Error::Type,
            }),
        }
    }
    pub fn bool(&self) -> ProgResult<bool> {
        match self {
            Self::Bool(v) => Ok(*v),
            _ => Err(Exception {
                msg: format!("{:?} is not a Bool!", self),
                error: Error::Type,
            }),
        }
    }
    pub fn list(&self) -> ProgResult<Vec<Atom>> {
        match self {
            Self::List(v) => Ok(v.clone()),
            _ => Err(Exception {
                msg: format!("{:?} is not a List!", self),
                error: Error::Type,
            }),
        }
    }
    pub fn string(&self) -> ProgResult<String> {
        match self {
            Self::String(v) => Ok(v.clone()),
            _ => Err(Exception {
                msg: format!("{:?} is not a String!", self),
                error: Error::Type,
            }),
        }
    }
    pub fn function(&self) -> ProgResult<Function> {
        match self {
            Self::Function(v) => Ok(v.clone()),
            _ => Err(Exception {
                msg: format!("{:?} is not a Function!", self),
                error: Error::Type,
            }),
        }
    }

    pub fn format(&self) -> String {
        match self {
            Atom::Bool(val) => val.to_string(),
            Atom::Function(val) => format!("{}()", val.name),
            Atom::Int(val) => val.to_string(),
            Atom::List(val) => format!(
                "[{}]",
                val.iter()
                    .map(|atom| atom.format())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Atom::Null => "null".to_string(),
            Atom::String(val) => val.clone(),
        }
    }
}
