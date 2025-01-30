/// Declares a group of builtin functions and exports them.
///
/// # Example
/// ```rust
/// use newlang::prelude::*;
/// functions! {
///     /// Length of a string.
///     "strlen"(1) => |state, args| {
///         let len = args[0].eval(state)?.string()?.len();
///         Ok(Atom::Int(len as i64))
///     }
///     /// Concatenate strings.
///     "strconcat"(_) => |state, args| {
///         let mut string = String::new();
///         for arg in args {
///             string.push_str(&arg.eval(state)?.string()?);
///         }
///         Ok(Atom::String(string))
///     }
///     /// Logical AND.
///     "&&"(2) => |state, args| Ok(Atom::Bool(
///         args[0].eval(state)?.bool()? &&
///         args[1].eval(state)?.bool()?
///     ))
///  }
/// ```
///
/// Here, the name before the parens is the function ident,
/// the parens contain the argc (`_` if any number of args is allowed)
/// and the right side is the closure body of the builtin function.
///
/// The macro invocation generates a `pub` function called `functions` that returns
/// `Vec<(&'static str, Function)>`.
#[macro_export]
macro_rules! functions {
    // TODO:
    //  in the past, `$name` was a `tt` and did not require to be quoted, but:
    //  this has problems when a name is multiple tokens wide (`&&`, `==` etc.).
    //  this is because `$name: tt` matches only one token and `$($name: tt)* would cause
    //  ambiguity errors when matching `(`
    //  also, `$name: tt` caused issues when trying to match `$(#[$doc: meta])`
    //  objective: fix these problems and use this again eventually as it is a nicer syntax
    //  compared to putting the value into a string literal
    ($(
        $(#[doc = $doc: literal])* $name: literal ($argc: tt) => $callback: expr)
    *) => {
        pub fn functions() -> Vec<(&'static str, $crate::prelude::Function)> {
            $(
                $crate::check_nonempty_doc! {
                    $(#[doc = $doc])* $name
                }
            )*
            vec![
                $((
                    $name,
                    $crate::prelude::Function {
                        doc: [$($doc),*].join("\n"),
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
macro_rules! check_nonempty_doc {
    ($name: literal) => {
        compile_error!(concat!(
            "builtin function ",
            stringify!($name),
            " has no documentation"
        ));
    };
    ($(#[doc = $doc: literal])* $name: literal) => {};
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
