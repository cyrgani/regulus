#[macro_export]
macro_rules! function {
    (
        name: $name: ident,
        argc: $argc: expr,
        callback: $callback: expr,
    ) => {
        fn $name() -> (&'static str, $crate::prelude::Function) {
            let mut name = stringify!($name);
            if let Some(stripped_name) = name.strip_prefix("r#") {
                name = stripped_name;
            }
            (
                name,
                $crate::prelude::Function {
                    argc: $argc,
                    callback: std::rc::Rc::new($callback),
                },
            )
        }
    };
}
