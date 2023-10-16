use std::{error::Error, fmt};

use std::collections::HashMap;

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

type Storage = HashMap<String, Value>;

#[derive(Debug, Clone)]
enum Token {
    Function(FunctionName),
    LeftParen,
    Comma,
    RightParen,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            ',' => Self::Comma,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
enum FunctionName {
    // Logical
    Or,
    And,
    // Comparative
    Less,
    Greater,
    LessEquals,
    GreaterEquals,
    // Branching
    If,
    IfElse,
    While,
    // Meta
    Run,
    Assign,
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    // Debug
    Debug,
    // Internals
    Variable(String),
    Value(Value),
    // List
    List,
    Index,
    Push,
}

impl TryFrom<&str> for FunctionName {
    type Error = ProgError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "or" => Self::Or,
            "and" => Self::And,
            "if" => Self::If,
            "ifelse" => Self::IfElse,
            "_" => Self::Run,
            "assign" => Self::Assign,
            "<" => Self::Less,
            "<=" => Self::LessEquals,
            ">" => Self::Greater,
            ">=" => Self::GreaterEquals,
            "debug" => Self::Debug,
            "while" => Self::While,
            "+" => Self::Add,
            "*" => Self::Multiply,
            "~" => Self::Subtract,
            "list" => Self::List,
            "index" => Self::Index,
            "push" => Self::Push,
            _ => return Err(ProgError(format!("Unknown function name '{value}'!"))),
        })
    }
}

impl FunctionName {
    fn argc(&self) -> Option<usize> {
        Some(match self {
            Self::Or => 2,
            Self::And => 2,
            Self::If => 2,
            Self::IfElse => 3,
            Self::Run => return None,
            Self::Assign => 2,
            Self::Less => 2,
            Self::LessEquals => 2,
            Self::Greater => 2,
            Self::GreaterEquals => 2,
            Self::Debug => 1,
            Self::While => 2,
            Self::Multiply => 2,
            Self::Add => 2,
            Self::Subtract => 2,
            Self::Variable(_) => 0,
            Self::Value(_) => 0,
            Self::List => return None,
            Self::Index => 2,
            Self::Push => 2,
        })
    }
}

#[derive(Debug, Clone)]
struct Function {
    function_name: FunctionName,
    arg_locations: Vec<usize>,
    parent: Option<usize>,
}

impl Function {
    fn eval(&self, functions: &[Function], storage: &mut Storage) -> ProgResult<Value> {
        if let Some(argc) = self.function_name.argc() {
            let arg_len = self.arg_locations.len();
            if argc != arg_len {
                return Err(ProgError(format!(
                    "expected `{argc}` args, found `{arg_len}` args for `{:?}`",
                    self.function_name
                )));
            }
        }

        let args = self
            .arg_locations
            .iter()
            .map(|&i| &functions[i])
            .collect::<Vec<_>>();

        // TODO should this be used?
        // It is shorter code but less efficient for functions like `ifelse` since it evaluated something unneded
        //let values = args[..self.function_name.argc().unwrap_or(0)].iter().map(|f| f.eval(functions, storage)).collect::<Vec<_>>();

        match &self.function_name {
            FunctionName::Or => Ok(Value::Bool(
                args[0].eval(functions, storage)?.bool()?
                    || args[1].eval(functions, storage)?.bool()?,
            )),
            FunctionName::And => Ok(Value::Bool(
                args[0].eval(functions, storage)?.bool()?
                    && args[1].eval(functions, storage)?.bool()?,
            )),
            FunctionName::If => Ok(if args[0].eval(functions, storage)?.bool()? {
                args[1].eval(functions, storage)?
            } else {
                Value::Null
            }),
            FunctionName::IfElse => Ok(if args[0].eval(functions, storage)?.bool()? {
                args[1].eval(functions, storage)?
            } else {
                args[2].eval(functions, storage)?
            }),
            FunctionName::Run => {
                /*let mut ret = Value::Null;
                for arg in args {
                    let val = arg.eval(functions, storage)?;
                }
                Ok(ret)*/
                for arg in args {
                    arg.eval(functions, storage)?;
                }
                Ok(Value::Null)
            }
            FunctionName::Assign => {
                if let FunctionName::Variable(var) = &functions[self.arg_locations[0]].function_name
                {
                    let val = args[1].eval(functions, storage)?;
                    storage.insert(var.clone(), val);
                    Ok(Value::Null)
                } else {
                    Err(ProgError(
                        "Error during assignment: no variable was found!".to_string(),
                    ))
                }
            }
            FunctionName::Less => Ok(Value::Bool(
                args[0].eval(functions, storage)?.int()?
                    < args[1].eval(functions, storage)?.int()?,
            )),
            FunctionName::LessEquals => Ok(Value::Bool(
                args[0].eval(functions, storage)?.int()?
                    <= args[1].eval(functions, storage)?.int()?,
            )),
            FunctionName::Greater => Ok(Value::Bool(
                args[0].eval(functions, storage)?.int()?
                    > args[1].eval(functions, storage)?.int()?,
            )),
            FunctionName::GreaterEquals => Ok(Value::Bool(
                args[0].eval(functions, storage)?.int()?
                    >= args[1].eval(functions, storage)?.int()?,
            )),
            FunctionName::Debug => {
                println!("{:?}", args[0].eval(functions, storage));
                Ok(Value::Null)
            }
            FunctionName::Add => {
                match args[0]
                    .eval(functions, storage)?
                    .int()?
                    .checked_add(args[1].eval(functions, storage)?.int()?)
                {
                    Some(i) => Ok(Value::Int(i)),
                    None => Err(ProgError("overflow occured during addition!".to_string())),
                }
            }
            FunctionName::Subtract => {
                match args[0]
                    .eval(functions, storage)?
                    .int()?
                    .checked_sub(args[1].eval(functions, storage)?.int()?)
                {
                    Some(i) => Ok(Value::Int(i)),
                    None => Err(ProgError(
                        "overflow occured during subtraction!".to_string(),
                    )),
                }
            }
            FunctionName::Multiply => {
                match args[0]
                    .eval(functions, storage)?
                    .int()?
                    .checked_mul(args[1].eval(functions, storage)?.int()?)
                {
                    Some(i) => Ok(Value::Int(i)),
                    None => Err(ProgError(
                        "overflow occured during multiplication!".to_string(),
                    )),
                }
            }
            FunctionName::While => {
                while args[0].eval(functions, storage)?.bool()? {
                    args[1].eval(functions, storage)?;
                }
                Ok(Value::Null)
            }
            FunctionName::Variable(var) => match storage.get(var) {
                Some(value) => Ok(value.clone()),
                None => Err(ProgError(format!("No variable named `{var}` found!"))),
            },
            FunctionName::Value(val) => Ok(val.clone()),
            FunctionName::List => {
                let mut list = vec![];
                for arg in args {
                    list.push(arg.eval(functions, storage)?);
                }
                Ok(Value::List(list))
            }
            FunctionName::Index => Ok(Value::Int(
                args[0]
                    .eval(functions, storage)?
                    .list()?
                    .get(args[1].eval(functions, storage)?.int()? as usize)
                    .ok_or(ProgError("Unable to index list!".to_string()))?
                    .int()?,
            )),
            FunctionName::Push => {
                args[0]
                    .eval(functions, storage)?
                    .list()?
                    .push(args[1].eval(functions, storage)?);
                Ok(Value::Null)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Null,
    List(Vec<Value>),
}

impl TryFrom<&str> for Value {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(int) = value.parse::<i32>() {
            Ok(Value::Int(int))
        } else {
            Ok(match value {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                "null" => Value::Null,
                _ => return Err(()),
            })
        }
    }
}

impl Value {
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
    fn list(&self) -> ProgResult<Vec<Value>> {
        match self {
            Self::List(v) => Ok(v.clone()),
            _ => Err(ProgError(format!("{:?} is not a List!", self))),
        }
    }
}

fn tokenize(code: &str) -> ProgResult<Vec<Token>> {
    let mut tokens = vec![];
    let mut current = String::new();

    for c in code.chars() {
        match c {
            '(' => {
                if !current.is_empty() {
                    tokens.push(Token::Function(FunctionName::try_from(current.as_str())?));
                    current.clear();
                }
                tokens.push(Token::from(c));
            }
            ')' | ',' => {
                if !current.is_empty() {
                    tokens.push(match Value::try_from(current.as_str()) {
                        Ok(value) => Token::Function(FunctionName::Value(value)),
                        Err(_) => Token::Function(FunctionName::Variable(current.clone())),
                    });
                    current.clear();
                }
                tokens.push(Token::from(c));
            }
            ' ' | '\n' | '\t' => (),
            _ => current.push(c),
        }
    }

    Ok(tokens)
}

fn build_functions(tokens: &[Token]) -> ProgResult<Vec<Function>> {
    let mut functions = vec![];

    let mut current = None;
    for token in tokens {
        match token {
            Token::Function(f) => {
                let new_id = functions.len();
                functions.push(Function {
                    function_name: f.clone(),
                    arg_locations: vec![],
                    parent: current,
                });
                if let Some(parent) = current {
                    functions[parent].arg_locations.push(new_id)
                }
            }
            Token::LeftParen => match functions.len() {
                0 => {
                    return Err(ProgError(
                        "found `LeftParen` without existing function!".to_string(),
                    ))
                }
                _ => current = Some(functions.len() - 1),
            },
            Token::Comma => (), // TODO
            Token::RightParen => match current {
                Some(i) => current = functions[i].parent,
                None => {
                    return Err(ProgError(
                        "found `RightParen` without existing function!".to_string(),
                    ))
                }
            },
        };
    }

    Ok(functions)
}

fn strip_comments(code: &str) -> String {
    code.split('\n')
        .map(|line| line.split('#').next().unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn parse(code: &str) -> ProgResult<Value> {
    let without_comments = strip_comments(code);

    let tokens = tokenize(&without_comments)?;

    let functions = build_functions(&tokens)?;
    let mut storage = HashMap::new();
    functions
        .first()
        .ok_or(ProgError("No function was found!".to_string()))?
        .eval(&functions, &mut storage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Reads a file and returns the content.
    fn read_file(file_path: &str) -> String {
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("No file {file_path} was found!"))
    }

    #[test]
    fn test_name() {
        dbg!(parse(&read_file("./programs/test.prog")));
    }
}
