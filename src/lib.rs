use std::{collections::HashMap, error, fmt, rc::Rc};

mod stdlib {
    pub mod core;
    pub mod io;
    pub mod list;
    pub mod logic;
    pub mod math;
    pub mod string;
}

mod prelude {
    pub use super::*;
    pub use std::rc::Rc;
    pub use ErrorClass::*;
}

use prelude::*;

#[derive(Debug)]
pub enum ErrorClass {
    TypeError,
    OverflowError,
    OtherError,
}

#[derive(Debug)]
pub struct ProgError {
    pub msg: String,
    pub class: ErrorClass,
}

impl fmt::Display for ProgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.class, self.msg)
    }
}

impl error::Error for ProgError {}

/// A shorthand alias for `Result<T, ProgError>`.
pub type ProgResult<T> = Result<T, ProgError>;

type Storage = HashMap<String, Atom>;

#[derive(Debug, Clone)]
enum Token {
    Function(String),
    LeftParen,
    Comma,
    RightParen,
    Atom(Atom),
    Name(String),
}

#[derive(Debug, Clone)]
struct FunctionCall {
    arg_locations: Vec<usize>,
    parent: Option<usize>,
    name: String,
}

type Callback = dyn Fn(&[Argument], &mut Storage, Vec<Argument>) -> ProgResult<Atom>;

#[derive(Clone)]
pub struct Function {
    name: String,
    argc: Option<usize>,
    callback: Rc<Callback>,
}

// the callback cannot be debugged
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("argc", &self.argc)
            .finish()
    }
}

#[derive(Debug, Clone)]
enum Argument {
    FunctionCall(FunctionCall),
    Atom(Atom),
    Variable(String),
}

impl Argument {
    fn eval(&self, program: &[Argument], storage: &mut Storage) -> ProgResult<Atom> {
        match self {
            Argument::FunctionCall(call) => call.eval(program, storage),
            Argument::Atom(atom) => Ok(atom.clone()),
            Argument::Variable(var) => match storage.get(var) {
                Some(value) => Ok(value.clone()),
                None => Err(ProgError {
                    msg: format!("No variable named `{var}` found!"),
                    class: OtherError,
                }),
            },
        }
    }
}

impl FunctionCall {
    fn eval(&self, program: &[Argument], storage: &mut Storage) -> ProgResult<Atom> {
        let function = get_function(&self.name, storage)?;

        if let Some(argc) = function.argc {
            let arg_len = self.arg_locations.len();
            if argc != arg_len {
                return Err(ProgError {
                    msg: format!(
                        "expected `{argc}` args, found `{arg_len}` args for `{:?}`",
                        function.name
                    ),
                    class: OtherError,
                });
            }
        }

        let args = self
            .arg_locations
            .iter()
            .map(|&i| program[i].clone())
            .collect::<Vec<_>>();

        (function.callback)(program, storage, args)
    }
}

#[derive(Debug, Clone)]
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
    fn int(&self) -> ProgResult<i32> {
        match self {
            Self::Int(v) => Ok(*v),
            _ => Err(ProgError {
                msg: format!("{:?} is not a Int!", self),
                class: TypeError,
            }),
        }
    }
    fn bool(&self) -> ProgResult<bool> {
        match self {
            Self::Bool(v) => Ok(*v),
            _ => Err(ProgError {
                msg: format!("{:?} is not a Bool!", self),
                class: TypeError,
            }),
        }
    }
    fn list(&self) -> ProgResult<Vec<Atom>> {
        match self {
            Self::List(v) => Ok(v.clone()),
            _ => Err(ProgError {
                msg: format!("{:?} is not a List!", self),
                class: TypeError,
            }),
        }
    }
    fn string(&self) -> ProgResult<String> {
        match self {
            Self::String(v) => Ok(v.clone()),
            _ => Err(ProgError {
                msg: format!("{:?} is not a String!", self),
                class: TypeError,
            }),
        }
    }

    fn format(&self) -> String {
        match self {
            Atom::Bool(val) => val.to_string(),
            Atom::Function(val) => "FunctionTODO".to_owned(),
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

fn get_function(name: &str, storage: &Storage) -> ProgResult<Function> {
    storage
        .values()
        .find_map(|atom| {
            if let Atom::Function(function) = atom {
                if function.name == name {
                    Some(function.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .ok_or(ProgError {
            msg: format!("No function `{name}` found!"),
            class: OtherError,
        })
}

fn tokenize(code: &str) -> ProgResult<Vec<Token>> {
    let mut tokens = vec![];
    let mut current = String::new();
    let mut in_string = false;

    for c in code.chars() {
        match c {
            '(' => {
                if in_string {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        tokens.push(Token::Function(current.clone()));
                        current.clear();
                    }
                    tokens.push(Token::LeftParen);
                }
            }
            ')' | ',' => {
                if in_string {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        tokens.push(match Atom::try_from(current.as_str()) {
                            Ok(value) => Token::Atom(value),
                            Err(_) => Token::Name(current.clone()),
                        });
                        current.clear();
                    }
                    tokens.push(match c {
                        ')' => Token::RightParen,
                        ',' => Token::Comma,
                        _ => unreachable!(),
                    });
                }
            }
            '"' => {
                if in_string {
                    tokens.push(Token::Atom(Atom::String(current.clone())));
                    current.clear();
                }
                in_string = !in_string;
            }
            ' ' | '\n' | '\t' => (),
            _ => current.push(c),
        }
    }

    Ok(tokens)
}

fn build_program(tokens: &[Token]) -> ProgResult<Vec<Argument>> {
    let mut program = vec![];

    let mut current = None;

    for token in tokens {
        match token {
            Token::Function(f) => {
                let new_id = program.len();
                program.push(Argument::FunctionCall(FunctionCall {
                    arg_locations: vec![],
                    parent: current,
                    name: f.clone(),
                }));
                if let Some(parent) = current {
                    if let Argument::FunctionCall(call) = &mut program[parent] {
                        call.arg_locations.push(new_id)
                    }
                }
            }
            Token::LeftParen => match program.len() {
                0 => {
                    return Err(ProgError {
                        msg: "found `LeftParen` without existing function!".to_string(),
                        class: OtherError,
                    })
                }
                _ => current = Some(program.len() - 1),
            },
            Token::Comma => (), // TODO
            Token::RightParen => match current {
                Some(i) => {
                    if let Argument::FunctionCall(call) = &program[i] {
                        current = call.parent
                    }
                }
                None => {
                    return Err(ProgError {
                        msg: "found `RightParen` without existing function!".to_string(),
                        class: OtherError,
                    })
                }
            },
            Token::Name(name) => {
                let new_id = program.len();
                program.push(Argument::Variable(name.clone()));
                if let Some(parent) = current {
                    if let Argument::FunctionCall(call) = &mut program[parent] {
                        call.arg_locations.push(new_id)
                    }
                }
            }
            Token::Atom(atom) => {
                let new_id = program.len();
                program.push(Argument::Atom(atom.clone()));
                if let Some(parent) = current {
                    if let Argument::FunctionCall(call) = &mut program[parent] {
                        call.arg_locations.push(new_id)
                    }
                }
            }
        };
    }

    Ok(program)
}

fn strip_comments(code: &str) -> String {
    code.split('\n')
        .map(|line| line.split('#').next().unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

fn all_functions() -> Vec<Function> {
    use stdlib::*;

    let mut functions = vec![];

    for module in [
        core::functions(),
        io::functions(),
        math::functions(),
        logic::functions(),
        list::functions(),
        string::functions(),
    ] {
        functions.extend(module)
    }

    functions
}

fn initial_storage() -> Storage {
    let functions = all_functions();

    let mut storage = HashMap::new();
    for function in functions {
        storage.insert(function.name.clone(), Atom::Function(function));
    }
    storage
}

pub fn run(code: &str) -> ProgResult<Atom> {
    let without_comments = strip_comments(code);

    let tokens = tokenize(&without_comments)?;

    let program = build_program(&tokens)?;

    let mut storage = initial_storage();

    program
        .first()
        .ok_or(ProgError {
            msg: "No function was found!".to_string(),
            class: OtherError,
        })?
        .eval(&program, &mut storage)
}
