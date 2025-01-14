macro_rules! collect_builtins {
    ($($name: ident),*) => {
        pub fn all_functions() -> std::collections::HashMap<String, $crate::Atom> {
            let mut functions = std::collections::HashMap::new();

            for module in [
                $($name::functions(),)*
            ] {
                for (name, function) in module {
                    functions.insert(name.to_string(), $crate::Atom::Function(function));
                }
            }

            functions
        }

    };
}

macro_rules! stl_modules {
    ($($name: ident),*) => {
        $(pub mod $name;)*
        collect_builtins! {
            $($name),*
        }
    };
}

stl_modules! {
    cast, core, help, io, list, logic, math, string, time
}
