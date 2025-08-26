use crate::exception::ArgumentError;
use crate::prelude::*;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub args: Vec<Argument>,
    pub name: String,
    pub doc_comment: String,
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
                raise!(
                    state,
                    ArgumentError,
                    "expected `{argc}` args, found `{arg_len}` args for `{}`",
                    self.name
                );
            }
        }

        function.call(state, &self.args, &self.name)
    }

    /// Returns an approximation of the source code of this function call.
    pub fn stringify(&self) -> String {
        format!(
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(Argument::stringify)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub type FunctionBody = dyn Fn(&mut State, &[Argument]) -> Result<Atom>;

#[derive(Clone)]
pub struct Function(Rc<FunctionInner>);

impl Function {
    pub fn new(doc: impl Into<String>, argc: Option<usize>, body: Box<FunctionBody>) -> Self {
        Self(Rc::new(FunctionInner {
            doc: doc.into(),
            argc,
            body,
        }))
    }

    pub fn doc(&self) -> &str {
        self.0.doc.as_str()
    }

    pub fn argc(&self) -> Option<usize> {
        self.0.argc
    }

    pub fn body(&self) -> &FunctionBody {
        &self.0.body
    }

    // TODO: consider making this public
    pub(crate) fn call(
        &self,
        state: &mut State,
        args: &[Argument],
        fn_name_hint: &str,
    ) -> Result<Atom> {
        if let Some(argc) = self.argc() {
            let arg_len = args.len();
            if argc != arg_len {
                raise!(
                    ArgumentError,
                    "expected `{argc}` args, found `{arg_len}` args for `{fn_name_hint}`",
                );
            }
        }
        (self.body())(state, args)
    }
}

struct FunctionInner {
    doc: String,
    argc: Option<usize>,
    body: Box<FunctionBody>,
}

// the callback cannot be debugged
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("doc", &self.0.doc)
            .field("argc", &self.0.argc)
            .field("body", &"..")
            .finish()
    }
}

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
