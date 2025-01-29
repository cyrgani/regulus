/// Declares a group of builtin functions and exports them into a `Vec`.
///
/// # Examples
/// ```rust
/// use newlang::prelude::*;
/// functions! {
///     strlen(1) => |state, args| {
///         let len = args[0].eval(state)?.string()?.len();
///         Ok(Atom::Int(len as i64))
///     }
///     strconcat(_) => |state, args| {
///         let mut string = String::new();
///         for arg in args {
///             string.push_str(&arg.eval(state)?.string()?);
///         }
///         Ok(Atom::String(string))
///     }
///  }
/// ```
/// Temporary syntax with string literals (see below):
/// ```rust
/// use newlang::prelude::*;
/// functions! {
///     "&&"(2) => |state, args| Ok(Atom::Bool(
///         args[0].eval(state)?.bool()? &&
///         args[1].eval(state)?.bool()?
///     ))
/// }
/// ```
///
/// Here, the name before the parens is the function ident,
/// the parens contain the argc (`_` if any number of args is allowed)
/// and the right side is the closure body of the builtin function.
///
/// Currently, some function names need to be passed as a string literal if they are multiple Rust
/// tokens wide. This will be fixed eventually.
#[macro_export]
macro_rules! functions {
    // TODO:
    //  this has problems when a name is multiple tokens wide (`&&`, `==` etc.).
    //  this is because `$name: tt` matches only one token and `$($name: tt)* would cause
    //  ambiguity errors when matching `(`
    //  objective: fix these problems and use this everywhere eventually as it is a nicer syntax
    //  compared to putting the value into a string literal
    ($(
        /*$(#[$doc: meta])**/ $name: tt ($argc: tt) => $callback: expr)
    *) => {
        pub fn functions() -> Vec<(&'static str, $crate::prelude::Function)> {
            vec![
                 $((
                    $crate::stringify_non_literals!($name),
                    $crate::prelude::Function {
                        doc: String::new(),
                        //doc: stringify!($($doc)*).to_string(),
                        argc: $crate::make_argc!($argc),
                        callback: std::rc::Rc::new($callback),
                    },
                )),*
            ]
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! make_argc {
    (_) => {
        None
    };
    ($num: literal) => {
        Some($num)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! stringify_non_literals {
    ($lit: literal) => {
        $lit
    };
    ($t: tt) => {
        stringify!($t)
    };
}
