use crate::exception::{ArgumentError, NameError, TypeError};
use crate::list::List;
use crate::parsing::Span;
use crate::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Argument {
    FunctionCall(FunctionCall, Span),
    Atom(Atom, Span),
    Variable(String, Span),
}

impl Argument {
    pub fn eval<'a>(&'a self, state: &'a mut State) -> Result<Cow<'a, Atom>> {
        if state.exit_unwind_value.is_some() {
            return Ok(Cow::Owned(Atom::Null));
        }
        state.backtrace.push(self.span().clone());
        if let Self::FunctionCall(call, _) = self {
            state.current_doc_comment = Some(call.doc_comment.clone());
            state.current_fn_name = Some(call.name.clone());
        } else {
            state.current_doc_comment = None;
            state.current_fn_name = None;
        }
        let res = match self {
            Self::FunctionCall(call, _) => call.eval(state).map(Cow::Owned),
            Self::Atom(atom, _) => Ok(Cow::Borrowed(atom)),
            Self::Variable(var, _) => match state.storage.get(var) {
                Some(value) => Ok(Cow::Borrowed(value)),
                None => raise!(state, NameError, "No variable named `{var}` found!"),
            },
        };
        state.backtrace.pop();
        res
    }

    /// Returns the identifier of this variable.
    /// If it is not a variable, it raises an exception with the given error message.
    pub(crate) fn variable(&self, error_msg: &str, state: &State) -> Result<&str> {
        match self {
            Self::Variable(var, _) => Ok(var),
            _ => raise!(state, ArgumentError, error_msg),
        }
    }

    /// Returns an approximation of the source code of this argument.
    pub fn stringify(&self) -> String {
        match self {
            Self::Atom(atom, _) => atom.stringify(),
            Self::FunctionCall(call, _) => call.stringify(),
            Self::Variable(name, _) => name.clone(),
        }
    }

    /// Returns the span of this argument.
    pub const fn span(&self) -> &Span {
        match self {
            Self::Atom(_, s) | Self::FunctionCall(_, s) | Self::Variable(_, s) => s,
        }
    }
}

macro_rules! argument_eval_as_methods {
    ($($method_name: ident: $variant:ident -> $ty:ty;)*) => {
        #[allow(dead_code, reason = "all methods just provided for completeness")]
        impl Argument {
            $(
                pub fn $method_name(&self, state: &mut State) -> Result<$ty> {
                    match self.eval(state)?.into_owned() {
                        Atom::$variant(v) => Ok(v),
                        val => raise!(state, TypeError, "{val} is not a {}", stringify!($variant)),
                    }
                }
            )*
        }
    };
}

impl Argument {
    pub fn eval_as_string(&self, state: &mut State) -> Result<String> {
        self.eval(state)?
            .as_string()
            .ok_or_else(|| state.raise(TypeError, "{val} is not a list of chars"))
    }
}

// method name, atom variant name, rust type
argument_eval_as_methods! {
    eval_int: Int -> i64;
    eval_bool: Bool -> bool;
    eval_char: Char -> char;
    eval_list: List -> List;
    eval_function: Function -> Function;
    eval_object: Object -> Object;
}
