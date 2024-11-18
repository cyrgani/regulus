use std::collections::HashMap;
use crate::prelude::*;
use crate::state::State;
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
    pub fn eval(&self, state: &mut State) -> ProgResult<Atom> {
        let function = state.get_function(&self.name)?;

        if let Some(argc) = function.argc {
            let arg_len = self.args.len();
            if argc != arg_len {
                return Exception::new_err(
                    format!(
                        "expected `{argc}` args, found `{arg_len}` args for `{}`",
                        self.name
                    ),
                    Error::Argument,
                );
            }
        }

        (function.callback)(state, &self.args)
    }
}

type Callback = dyn Fn(&mut State, &[Argument]) -> ProgResult<Atom>;

#[derive(Clone)]
pub struct Function {
    #[deprecated]
    pub name: String,
    #[deprecated]
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
    fn eq(&self, _other: &Self) -> bool {
        false
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

    pub fn new_group(names: &[&str], argc: Option<usize>, callback: Rc<Callback>) -> Vec<Self> {
        names
            .iter()
            .map(|name| Self {
                name: String::from("deprecated"),
                aliases: vec![],
                argc,
                callback: callback.clone(),
            })
            .collect()
    }
}

pub fn all_functions() -> HashMap<String, Atom> {
    #[allow(clippy::wildcard_imports, reason = "more practical")]
    use crate::stdlib::*;

    let mut functions = HashMap::new();

    for module in [
        cast::functions(),
        core::functions(),
        io::functions(),
        math::functions(),
        logic::functions(),
        list::functions(),
        string::functions(),
        time::functions(),
    ] {
        for (name, function) in module {
            functions.insert(name.to_string(), Atom::Function(function));
        }
    }

    functions
}
