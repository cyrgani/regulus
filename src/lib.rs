use std::{collections::HashMap, error::Error, fmt};

mod modules {
    pub mod core;
    pub mod debug;
    pub mod list;
    pub mod logic;
    pub mod math;
}

/// A error type that should be returned by interpreters.
/// Contains a message that is a `String`.
#[derive(Debug)]
pub struct ProgError(pub String);

impl fmt::Display for ProgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ProgError {}

/// A shorthand alias for `Result<T, ProgError>`.
pub type ProgResult<T> = Result<T, ProgError>;

type Storage = HashMap<String, Atom>;

#[derive(Debug, Clone)]
enum Token {
    Function(Function),
    LeftParen,
    Comma,
    RightParen,
    Quote,
    Atom(Atom),
    Name(String),
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            ',' => Self::Comma,
            '"' => Self::Quote,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct FunctionCall {
    arg_locations: Vec<usize>,
    parent: Option<usize>,
    function: Function,
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    argc: Option<usize>,
    callback:
        fn(program: &[Argument], storage: &mut Storage, args: Vec<Argument>) -> ProgResult<Atom>,
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
                None => Err(ProgError(format!("No variable named `{var}` found!"))),
            },
        }
    }
}

static mut FUNCTIONS: Vec<Function> = vec![];

impl FunctionCall {
    fn eval(&self, program: &[Argument], storage: &mut Storage) -> ProgResult<Atom> {
        if let Some(argc) = self.function.argc {
            let arg_len = self.arg_locations.len();
            if argc != arg_len {
                return Err(ProgError(format!(
                    "expected `{argc}` args, found `{arg_len}` args for `{:?}`",
                    self.function.name
                )));
            }
        }

        let args = self
            .arg_locations
            .iter()
            .map(|&i| program[i].clone())
            .collect::<Vec<_>>();

        return (self.function.callback)(program, storage, args);
    }
}

fn get_function(name: &str) -> ProgResult<Function> {
    unsafe {
        FUNCTIONS
            .iter()
            .find(|function| function.name == name)
            .ok_or(ProgError(format!("No function `{name}` found!")))
            .cloned()
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Int(i32),
    Bool(bool),
    Null,
    List(Vec<Atom>),
    String(String),
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
            _ => Err(ProgError(format!("{:?} is not an Int!", self))),
        }
    }
    fn bool(&self) -> ProgResult<bool> {
        match self {
            Self::Bool(v) => Ok(*v),
            _ => Err(ProgError(format!("{:?} is not a Bool!", self))),
        }
    }
    fn list(&self) -> ProgResult<Vec<Atom>> {
        match self {
            Self::List(v) => Ok(v.clone()),
            _ => Err(ProgError(format!("{:?} is not a List!", self))),
        }
    }
    fn string(&self) -> ProgResult<String> {
        match self {
            Self::String(v) => Ok(v.clone()),
            _ => Err(ProgError(format!("{:?} is not a String!", self))),
        }
    }
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
                        tokens.push(Token::Function(get_function(&current)?));
                        current.clear();
                    }
                    tokens.push(Token::from(c));
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
                    tokens.push(Token::from(c));
                }
            }
            '"' => {
                if in_string {
                    tokens.push(Token::Atom(Atom::String(current.clone())));
                }
                in_string = !in_string;
                //tokens.push(Token::Quote)
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
                    function: f.clone(),
                }));
                if let Some(parent) = current {
                    if let Argument::FunctionCall(call) = &mut program[parent] {
                        call.arg_locations.push(new_id)
                    }
                }
            }
            Token::LeftParen => match program.len() {
                0 => {
                    return Err(ProgError(
                        "found `LeftParen` without existing function!".to_string(),
                    ))
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
                    return Err(ProgError(
                        "found `RightParen` without existing function!".to_string(),
                    ))
                }
            },
            Token::Quote => (),
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

fn add_functions() {
    use modules::*;

    let mut functions = vec![];
    for module in [
        core::functions(),
        debug::functions(),
        math::functions(),
        logic::functions(),
        list::functions(),
    ] {
        functions.extend(module)
    }

    unsafe {
        FUNCTIONS = functions;
    }
}

pub fn run(code: &str) -> ProgResult<Atom> {
    add_functions();

    let without_comments = strip_comments(code);

    let tokens = tokenize(&without_comments)?;

    let program = build_program(&tokens)?;

    dbg!(&program);
    let mut storage = HashMap::new();
    program
        .first()
        .ok_or(ProgError("No function was found!".to_string()))?
        .eval(&program, &mut storage)
}
