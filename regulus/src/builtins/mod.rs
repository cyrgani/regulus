macro_rules! builtin_modules {
    ($($name: ident),*) => {
        $(pub mod $name;)*

        pub fn all_functions() -> std::collections::HashMap<String, $crate::Atom> {
            let mut functions = std::collections::HashMap::new();

            $(for (name, function) in $name::functions() {
                functions.insert(name.to_string(), $crate::Atom::Function(function));
            })*

            functions
        }
    };
}

builtin_modules! {
    cast, core, help, io, list, logic, math, string, time
}
