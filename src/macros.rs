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

/// Advantages of making this a proc macro:
/// 1. any function name is supported, even `if` or `<`
/// 2. `export!` is not required anymore
///
/// Disadvantages:
/// 1. either `syn` as a library dep (not nice but ok)
/// 2. less syntax suggestions (but few are possible anyway)
/// 3. worse diagnostics (?), maybe better in some cases?
#[macro_export]
macro_rules! functions {
    /*($($name: tt (_) => $callback: expr)*) => {
        /*pub fn functions() -> Vec<(&'static str, $crate::prelude::Function)> {

        }*/
        $(function! {
            name: $name,
            override_name: $name,
            argc: None,
            callback: $callback,
        })*
    };*/
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
        /*$(function! {
            name: $name,
            override_name: $name,
            argc: Some($argc),
            callback: $callback,
        })**/
    };
}
