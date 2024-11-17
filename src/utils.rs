#[macro_export]
macro_rules! function {
    (
        name: $name: ident,
        argc: $argc: expr,
        callback: $callback: expr,
    ) => {
        function! {
            name: $name,
            aliases: vec![],
            argc: $argc,
            callback: $callback,
        }
    };
    (
        name: $name: ident,
        aliases: $aliases: expr,
        argc: $argc: expr,
        callback: $callback: expr,
    ) => {
        fn $name() -> $crate::prelude::Function {
            let mut name = stringify!($name);
            if let Some(stripped_name) = name.strip_prefix("r#") {
                name = stripped_name;
            }
            $crate::prelude::Function {
                name: String::from(name),
                aliases: $aliases,
                argc: $argc,
                callback: std::rc::Rc::new($callback),
            }
        }
    };
}
