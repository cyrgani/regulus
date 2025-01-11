#[deprecated]
/// Declares a builtin function.
///
/// # Examples
/// ```rust
/// use newlang::prelude::*;
/// function! {
///     name: not,
///     argc: Some(1),
///     callback: |state, args| Ok(Atom::Bool(!args[0].eval(state)?.bool()?)),
/// }
/// ```
///
/// This macro also supports a name override syntax to create functions named like Rust tokens:
/// ```rust
/// use newlang::prelude::*;
/// function! {
///     name: not,
///     override_name: !,
///     argc: Some(1),
///     callback: |state, args| Ok(Atom::Bool(!args[0].eval(state)?.bool()?)),
/// }
/// ```
///
/// In this case, `name` is just used once as an identifier to `export!` the function.
#[macro_export]
macro_rules! function {
    (
        name: $name: ident,
        argc: $argc: expr,
        callback: $callback: expr,
    ) => {
        function! {
            name: $name,
            override_name: $name,
            argc: $argc,
            callback: $callback,
        }
    };
    (
        name: $name: ident,
        override_name: $override_name: tt,
        argc: $argc: expr,
        callback: $callback: expr,
    ) => {
        fn $name() -> (&'static str, $crate::prelude::Function) {
            (
                stringify!($override_name),
                $crate::prelude::Function {
                    argc: $argc,
                    callback: std::rc::Rc::new($callback),
                },
            )
        }
    };
}

#[deprecated]
#[macro_export]
macro_rules! export {
    ($($func: ident),*,) => {
        pub fn functions() -> Vec<(&'static str, $crate::prelude::Function)> {
            vec![
                $($func(),)*
            ]
        }
    };
}

#[macro_export]
macro_rules! make_argc {
    (_) => {
        None
    };
    ($num: literal) => {
        Some($num)
    };
}

/// Declares a group of builtin functions and exports them into a `Vec`.
///
/// # Example
/// ```rust
/// use newlang::prelude::*;
/// functions! {
///     !(1) => |state, args| Ok(Atom::Bool(!args[0].eval(state)?.bool()?))
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
    // TODO: workaround, see below
    ($($name: literal ($argc: tt) => $callback: expr)*) => {
        pub fn functions() -> Vec<(&'static str, $crate::prelude::Function)> {
            vec![
                 $((
                    $name,
                    $crate::prelude::Function {
                        argc: $crate::make_argc!($argc),
                        callback: std::rc::Rc::new($callback),
                    },
                )),*
            ]
        }
    };
    // TODO: 
    //  this has problems when a name is multiple tokens wide (`&&`, `==` etc.).
    //  this is because `$name: tt` matches only one token and `$($name: tt)* would cause
    //  ambiguity errors when matching `(`
    //  objective: fix these problems and use this everywhere eventually as it is a nicer syntax
    ($($name: tt ($argc: tt) => $callback: expr)*) => {
        pub fn functions() -> Vec<(&'static str, $crate::prelude::Function)> {
            vec![
                 $((
                    stringify!($name),
                    $crate::prelude::Function {
                        argc: $crate::make_argc!($argc),
                        callback: std::rc::Rc::new($callback),
                    },
                )),*
            ]
        }
    };
}
