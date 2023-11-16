use std::{collections::HashMap, error, fmt, rc::Rc};

mod stdlib {
    pub mod cast;
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
    NameError,
    SyntaxError,
    ArgumentError,
    AssignError,
    IndexError,
    IoError,
    ImportError,
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
    args: Vec<Argument>,
    name: String,
}

impl FunctionCall {
    fn eval(&self, storage: &mut Storage) -> ProgResult<Atom> {
        let function = get_function(&self.name, storage)?;

        if let Some(argc) = function.argc {
            let arg_len = self.args.len();
            if argc != arg_len {
                return Err(ProgError {
                    msg: format!(
                        "expected `{argc}` args, found `{arg_len}` args for `{:?}`",
                        function.name
                    ),
                    class: ArgumentError,
                });
            }
        }

        (function.callback)(storage, &self.args)
    }
}

type Callback = dyn Fn(&mut Storage, &[Argument]) -> ProgResult<Atom>;

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
    fn eval(&self, storage: &mut Storage) -> ProgResult<Atom> {
        match self {
            Argument::FunctionCall(call) => call.eval(storage),
            Argument::Atom(atom) => Ok(atom.clone()),
            Argument::Variable(var) => match storage.get(var) {
                Some(value) => Ok(value.clone()),
                None => Err(ProgError {
                    msg: format!("No variable named `{var}` found!"),
                    class: NameError,
                }),
            },
        }
    }

    fn as_call(&self) -> ProgResult<&FunctionCall> {
        match self {
            Argument::FunctionCall(call) => Ok(call),
            _ => Err(ProgError {
                msg: "expected an function call which didn't exist".to_string(),
                class: TypeError,
            }),
        }
    }
    fn as_call_mut(&mut self) -> ProgResult<&mut FunctionCall> {
        match self {
            Argument::FunctionCall(call) => Ok(call),
            _ => Err(ProgError {
                msg: "expected an function call which didn't exist".to_string(),
                class: TypeError,
            }),
        }
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
    fn function(&self) -> ProgResult<Function> {
        match self {
            Self::Function(v) => Ok(v.clone()),
            _ => Err(ProgError {
                msg: format!("{:?} is not a Function!", self),
                class: TypeError,
            }),
        }
    }

    fn format(&self) -> String {
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
            class: NameError,
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
                    if current.is_empty() {
                        return Err(ProgError {
                            msg: "Found LeftParen without a function".to_string(),
                            class: SyntaxError,
                        });
                    } else {
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

fn build_program(tokens: Vec<Token>) -> ProgResult<FunctionCall> {
    let mut master = FunctionCall {
        name: "_".to_string(),
        args: vec![Argument::FunctionCall(FunctionCall {
            name: "_".to_string(),
            args: vec![],
        })],
    };

    let mut current_parent = &mut master;
    let mut current = current_parent.args.last_mut().unwrap().as_call_mut()?;

    for token in tokens.into_iter() {
        match token {
            Token::Atom(atom) => {
                current.args.push(Argument::Atom(atom));
            }
            Token::Name(name) => {
                current.args.push(Argument::Variable(name));
            }
            Token::Function(name) => {
                current
                    .args
                    .push(Argument::FunctionCall(FunctionCall { args: vec![], name }));
            }
            Token::Comma => (),
            Token::LeftParen => match current.args.last_mut().expect("tokens should allow this") {
                Argument::FunctionCall(call) => current = call,
                _ => {
                    return Err(ProgError {
                        msg: "LeftParen following no function call".to_string(),
                        class: SyntaxError,
                    })
                }
            },
            Token::RightParen => todo!()
        }
    }

    Ok(master)
}
/*
fn build_program(tokens: &[Token]) -> ProgResult<Argument> {
    let mut current = None;
    let mut first = None;
    let mut counter = 0;

    for token in tokens {
        match token {
            Token::Function(f) => {
                let new_id = counter;
                let function_call = Argument::FunctionCall(FunctionCall {
                    args: vec![],
                    parent: current,
                    name: f.clone(),
                });
                counter += 1;
                if let Some(parent) = current {
                    parent.args.push(function_call)
                } else {
                    first = Some(function_call);
                }
            }
            Token::LeftParen => match counter {
                0 => {
                    return Err(ProgError {
                        msg: "found `LeftParen` without existing function!".to_string(),
                        class: SyntaxError,
                    })
                }
                _ => current = current,
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
                        class: SyntaxError,
                    })
                }
            },
            Token::Name(name) => {
                let new_id = counter;
                let variable = Argument::Variable(name.clone());
                counter += 1;

                if let Some(parent) = current {
                    if let Argument::FunctionCall(call) = &mut program[parent] {
                        call.args.push(new_id)
                    }
                }
            }
            Token::Atom(atom) => {
                let new_id = counter;
                let atom = Argument::Atom(atom.clone());
                counter += 1;

                if let Some(parent) = current {
                    if let Argument::FunctionCall(call) = &mut program[parent] {
                        call.args.push(new_id)
                    }
                }
            }
        };
    }

    match first {
        Some(call) => Ok(call),
        None => Err(ProgError {
            msg: "No program start was found!".to_string(),
            class: SyntaxError,
        })
    }
}*/

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
        cast::functions(),
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

pub fn run(code: &str) -> ProgResult<(Atom, Storage)> {
    let without_comments = strip_comments(code);

    let tokens = tokenize(&without_comments)?;

    let program = build_program(tokens)?;

    let mut storage = initial_storage();

    let result = program.eval(&mut storage)?;
    Ok((result, storage))
}
