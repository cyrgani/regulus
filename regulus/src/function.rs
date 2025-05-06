use crate::prelude::*;
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
    pub fn eval(&self, state: &mut State) -> Result<Atom> {
        if state.exit_unwind_value.is_some() {
            return Ok(Atom::Null);
        }
        let function = state.get_function(&self.name)?;

        if let Some(argc) = function.argc {
            let arg_len = self.args.len();
            if argc != arg_len {
                return raise!(
                    Error::Argument,
                    "expected `{argc}` args, found `{arg_len}` args for `{}`",
                    self.name
                );
            }
        }

        (function.callback)(state, &self.args)
    }
}

type Callback = dyn Fn(&mut State, &[Argument]) -> Result<Atom>;

pub type Function = Rc<FunctionInner>;

pub struct FunctionInner {
    pub doc: String,
    pub argc: Option<usize>,
    pub callback: Box<Callback>,
}

// the callback cannot be debugged
impl fmt::Debug for FunctionInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("doc", &self.doc)
            .field("argc", &self.argc)
            .field("callback", &"..")
            .finish()
    }
}

impl PartialEq for FunctionInner {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
