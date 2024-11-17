#[macro_export]
macro_rules! function {
    (
        aliases: $aliases: expr,
        name: $name: ident,
        argc: $argc: expr,
        callback: $callback: expr,
    ) => {
        fn $name() -> $crate::prelude::Function {
            let mut name = stringify!($name);
            if let Some(stripped_name) = name.strip_prefix("r#") {
                name = stripped_name;
            }
            Function {
                aliases: $aliases,
                name: String::from(name),
                argc: $argc,
                callback: std::rc::Rc::new($callback),
            }
        }
    };
}