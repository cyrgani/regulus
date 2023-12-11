use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub args: Vec<Argument>,
    pub name: String,
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl FunctionCall {
    pub fn eval(&self, storage: &mut Storage) -> ProgResult<Atom> {
        let function = crate::storage::get_function(&self.name, storage)?;

        if let Some(argc) = function.argc {
            let arg_len = self.args.len();
            if argc != arg_len {
                return Err(Exception {
                    msg: format!(
                        "expected `{argc}` args, found `{arg_len}` args for `{}`",
                        function.name
                    ),
                    error: Error::Argument,
                });
            }
        }

        (function.callback)(storage, &self.args)
    }
}

type Callback = dyn Fn(&mut Storage, &[Argument]) -> ProgResult<Atom>;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub aliases: Vec<String>,
    pub argc: Option<usize>,
    pub callback: Rc<Callback>,
}

// the callback cannot be debugged
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("argc", &self.argc)
            .field("aliases", &self.aliases)
            .field("callback", &Rc::new(()))
            .finish()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Function {
    pub fn new(names: &[&str], argc: Option<usize>, callback: Rc<Callback>) -> Self {
        Self {
            name: names[0].to_string(),
            aliases: names[1..].iter().map(ToString::to_string).collect(),
            argc,
            callback,
        }
    }
}

pub fn all_functions() -> Vec<Function> {
    use crate::stdlib::*;

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
        for function in module {
            functions.push(function.clone());
            for alias in &function.aliases {
                let mut new = function.clone();
                new.name = alias.to_string();
                functions.push(new);
            }
        }
    }

    functions
}
