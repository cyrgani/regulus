use crate::prelude::*;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
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
        let function = state.storage.get_function(&self.name)?;

        if let Some(argc) = function.argc() {
            let arg_len = self.args.len();
            if argc != arg_len {
                return raise!(
                    Error::Argument,
                    "expected `{argc}` args, found `{arg_len}` args for `{}`",
                    self.name
                );
            }
        }

        (function.callback())(state, &self.args)
    }
}

type Callback = dyn Fn(&mut State, &[Argument]) -> Result<Atom>;

#[derive(Clone)]
pub struct Function(Rc<FunctionInner>);

impl Function {
    pub fn new(doc: impl Into<String>, argc: Option<usize>, callback: Box<Callback>) -> Self {
        Self(Rc::new(FunctionInner {
            doc: doc.into(),
            argc,
            callback,
        }))
    }

    pub fn doc(&self) -> &str {
        self.0.doc.as_str()
    }

    pub fn argc(&self) -> Option<usize> {
        self.0.argc
    }

    pub fn callback(&self) -> &Callback {
        &self.0.callback
    }
}

struct FunctionInner {
    doc: String,
    argc: Option<usize>,
    callback: Box<Callback>,
}

// the callback cannot be debugged
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("doc", &self.0.doc)
            .field("argc", &self.0.argc)
            .field("callback", &"..")
            .finish()
    }
}

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
