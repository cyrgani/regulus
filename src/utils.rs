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
            let mut real_name = stringify!($override_name);
            if let Some(stripped_name) = real_name.strip_prefix("r#") {
                real_name = stripped_name;
            }
            (
                real_name,
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
