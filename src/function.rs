use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub args: Vec<Argument>,
    pub name: String,
}

#[cfg(feature = "display_impls")]
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
    pub argc: Option<usize>,
    pub callback: Rc<Callback>,
}

// the callback cannot be debugged
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("argc", &self.argc)
            .field("callback", &"..")
            .finish()
    }
}

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

pub fn all_functions() -> HashMap<String, Atom> {
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
