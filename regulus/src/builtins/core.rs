use crate::interned_stdlib::INTERNED_STL;
use crate::prelude::*;
use crate::{Directory, FILE_EXTENSION};
use std::borrow::Cow;
use std::fs;
use std::rc::Rc;

fn define_function(body: &Argument, fn_args: &[Argument]) -> Result<Atom> {
    let body = body
        .function_call("Error during definition: no valid function body was given!")?
        .clone();
    let function_arg_names = fn_args
        .iter()
        .map(|fn_arg| {
            fn_arg
                .variable("Error during definition: invalid args were given!")
                .cloned()
        })
        .collect::<Result<Vec<_>>>()?;

    let function = FunctionInner {
        doc: String::new(),
        argc: Some(function_arg_names.len()),
        callback: Box::new(move |state, args| {
            // a function call should have its own scope and not leak variables
            // except for globals

            // TODO:
            //  this cloning of the whole storage is extremely inefficient
            //  a better idea would be a "tagged" storage (??)
            //  or: create a new empty storage, put all redefined vars in the function into it, but
            //      allow reading from both new and then old in that order
            //      problem: `body.eval(state);` can only take one state, not two
            let mut old_storage_data = state.storage.data.clone();

            for (idx, arg) in function_arg_names.iter().enumerate() {
                let arg_result = args[idx].eval(state)?.into_owned();
                state.storage.insert(arg.clone(), arg_result);
            }

            let function_result = body.eval(state);

            old_storage_data.extend(state.storage.global_items());

            state.storage.data = old_storage_data;
            function_result
        }),
    };

    Ok(Atom::Function(Rc::new(function)))
}

fn try_resolve_import_in_dir(name: &str, dir_path: &Directory) -> Option<String> {
    match &dir_path {
        Directory::Regular(path) => {
            let paths = fs::read_dir(path)
                .unwrap_or_else(|err| {
                    panic!("error when reading directory `{}`: {err}", path.display())
                })
                .flatten();
            for item in paths {
                if *item.file_name() == *format!("{name}.{FILE_EXTENSION}") {
                    if let Ok(file_content) = fs::read_to_string(item.path()) {
                        return Some(file_content);
                    }
                }
            }
            None
        }
        Directory::InternedSTL => INTERNED_STL.get(name).map(ToString::to_string),
    }
}

functions! {
    /// Evaluates all given arguments and returns the atom the last argument evaluated to.
    /// If no arguments are given, `null` is returned.
    ///
    /// Every program is implicitly wrapped in a call to this function.
    ///
    /// This function has an alias: `run`.
    "_"(_) => |state, args| {
        if args.is_empty() {
            Ok(Atom::Null)
        } else {
            for arg in &args[0..args.len() - 1] {
                arg.eval(state)?;
            }
            args[args.len() - 1].eval(state).map(Cow::into_owned)
        }
    }
    /// Assigns the second argument to a variable named like the first argument.
    ///
    /// This function has an alias: `assign`.
    "="(2) => |state, args| {
        let var = args[0].variable("Error during assignment: no variable was given to assign to!")?;
        let value = args[1].eval(state)?.into_owned();
        state.storage.insert(var, value);
        Ok(Atom::Null)
    }
    /// Evaluates the first argument as a boolean.
    /// If it evaluates to true, the second argument is evaluated and returned.
    /// If it evaluates to false, the second argument is ignored and `null` is returned.
    "if"(2) => |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?.into_owned()
        } else {
            Atom::Null
        })
    }
    /// Evaluates the first argument as a boolean.
    /// If it evaluates to true, the second argument is evaluated and returned.
    /// If it evaluates to false, the third argument is evaluated and returned instead.
    "ifelse"(3) => |state, args| {
        Ok(if args[0].eval(state)?.bool()? {
            args[1].eval(state)?
        } else {
            args[2].eval(state)?
        }.into_owned())
    }
    /// Repeatedly evaluates the first argument as a boolean.
    /// If it evaluates to true, the second argument is evaluated and the same steps begin again.
    /// If it evaluates to false, the loop ends and `null` is returned.
    "while"(2) => |state, args| {
        while args[0].eval(state)?.bool()? {
            args[1].eval(state)?;
        }
        Ok(Atom::Null)
    }
    /// Defines a new function.
    /// The first argument is the function identifier and the last argument is the function body.
    /// All arguments in between are the names of the function arguments that can be accessed in
    /// the function body.
    /// Values defined in the function are scoped and cannot be accessed outside of the function body.
    "def"(_) => |state, args| {
        let [var, fn_args @ .., body] = args else {
            return raise!(
                Error::Argument,
                "too few arguments passed to `def`: expected at least 2, found {}", args.len()
            );
        };
        let var = var.variable("Error during function definition: no valid variable was given to define to!")?;

        state.storage.insert(var, define_function(body, fn_args)?);
        Ok(Atom::Null)
    }
    /// Creates a new function and returns it.
    ///
    /// The last argument is the function body.
    /// All arguments before are the names of the function arguments that can be accessed in
    /// the function body.
    /// Values defined in the function are scoped and cannot be accessed outside of the function body.
    "fn"(_) => |_, args| {
        let Some((body, fn_args)) = args.split_last() else {
            return raise!(Error::Argument, "`fn` invocation is missing body");
        };
        define_function(body, fn_args)
    }
    /// Imports a file, either from the stl or the local directory.
    /// TODO document the exact algorithm and hierarchy more clearly, also the return value of this function
    "import"(1) => |state, args| {
        let name = args[0].variable("`import` argument must be a variable, string syntax was removed")?;
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return raise!(
                Error::Import,
                "invalid characters in import name `{name}`",
            );
        }

        // lookup order:
        // 1. look inside the programs current directory
        // 2. look in the global stl directory
        let source = if let Some(code) = try_resolve_import_in_dir(name, &state.file_directory) {
            Some((code, state.file_directory.clone()))
        } else {
            try_resolve_import_in_dir(name, &Directory::InternedSTL).map(|code| (code, Directory::InternedSTL))
        };

        let Some((code, source_dir)) = source else {
            return raise!(
                Error::Import,
                "failed to find file for importing `{name}`",
            );
        };

        // TODO: consider using `.file()` here instead
        let mut import_start_state = State::initial_with_dir(source_dir);
        import_start_state.storage.global_idents.clone_from(&state.storage.global_idents);
        import_start_state.storage.data.extend(state.storage.global_items());
        let (atom, imported_state) = Runner::new().code(code).starting_state(import_start_state).run();

        if let Some(exit_unwind_value) = imported_state.exit_unwind_value {
            state.exit_unwind_value = Some(exit_unwind_value);
            return Ok(Atom::Null);
        }
        let atom = atom?;

        for (k, v) in imported_state.storage.data {
            state.storage.insert(k, v);
        }
        state.storage.global_idents = imported_state.storage.global_idents;
        Ok(atom)
    }
    /// Raises an exception of the kind `UserRaised` with the given string message.
    "error"(1) => |state, args| {
        Err(Exception::new(args[0].eval(state)?.string()?, Error::UserRaised))
    }
    /// Evaluates the given value and returns it.
    /// If an exception occurs while evaluating the argument, the exception is converted into a
    /// string and returned instead.
    "catch"(1) => |state, args| {
        Ok(args[0]
            .eval(state).map_or_else(|exc| Atom::String(exc.to_string()), Cow::into_owned))
    }
    /// Evaluates both arguments and returns whether they are equal.
    /// TODO: define this behavior in edge cases and document it
    "=="(2) => |state, args| {
        Ok(Atom::Bool(args[0].eval(state)?.into_owned() == *args[1].eval(state)?))
    }
    /// Evaluates the argument as a boolean and returns `null` if it is true.
    /// If it is false, raise an exception of the `Assertion` kind.
    "assert"(1) => |state, args| {
        if args[0].eval(state)?.bool()? {
            Ok(Atom::Null)
        } else {
            raise!(Error::Assertion, "Assertion failed!")
        }
    }
    /// Evaluates both arguments and compares then, returning `null` if they are equal.
    /// If not, raise an exception of the `Assertion` kind with a message containing both values.
    "assert_eq"(2) => |state, args| {
        // FIXME: first `into_owned` is needed right now since eval is
        //  fn eval<'a>(&'a self, state: &'a mut State) -> Result<Cow<'a, Atom>>;
        let lhs = args[0].eval(state)?.into_owned();
        let rhs = args[1].eval(state)?;
        if lhs == *rhs {
            Ok(Atom::Null)
        } else {
            raise!(
                Error::Assertion,
                "Equality assertion failed! lhs: `{lhs}`, rhs: `{rhs}`!"
            )
        }
    }
    /// Defines a new type.
    /// The first argument must be given and is the ident of the type.
    /// All further arguments are its fields.
    "type"(_) => |state, args| {
        let Some((ident, fields)) = args.split_first() else {
            return raise!(Error::Argument, "`type` takes at least one argument");
        };
        let var = ident.variable("`type` must take a variable as first argument")?;
        let fields = fields
            .iter()
            .map(|field| field.variable("`type` field arguments should be variables").cloned())
            .collect::<Result<Vec<_>>>()?;

        let function = FunctionInner {
            argc: Some(fields.len()),
            doc: String::new(),
            callback: Box::new(move |state, args| {
                Ok(Atom::Object(
                    fields
                        .iter()
                        .zip(args)
                        .map(|(field, arg)| Ok((field.clone(), arg.eval(state)?.into_owned())))
                        .collect::<Result<_>>()?,
                ))
            }),
        };

        state.storage.insert(var, Atom::Function(Rc::new(function)));
        Ok(Atom::Null)
    }
    /// Get the value of a field of an object.
    ///
    /// The first argument is the object, the second is its name as a variable.
    /// TODO: consider allowing the second to also be an arg that evals to Atom::String?
    ///
    /// If the field does not exist on the object, an exception is raised.
    ///
    /// This function has an alias: `getattr`.
    "."(2) => |state, args| {
        let obj = args[0].eval(state)?.object()?;
        let field = args[1].variable("`.` takes a field identifier as second argument")?;
        obj.get(field).cloned().ok_or_else(|| Exception::new(format!("object has no field named `{field}`"), Error::Name))
    }
    /// Set the value of a field of an object to a new value and returns the updated object.
    ///
    /// The first argument is the object, the second is its name as a variable and the third is the new value.
    ///
    /// If the field does not exist on the object, an exception is raised.
    /// TODO: think if it should be allowed to add fields with this that did not exist before or not
    /// TODO: consider allowing the second to also be an arg that evals to Atom::String?
    ///
    /// This function has an alias: `setattr`.
    "->"(3) => |state, args| {
        let mut obj = args[0].eval(state)?.object()?;
        let field = args[1].variable("`.` takes a field identifier as second argument")?;
        let value = args[2].eval(state)?;
        *obj.get_mut(field).ok_or_else(|| Exception::new(format!("object has no field named `{field}`"), Error::Name))? = value.into_owned();
        Ok(Atom::Object(obj))
    }
    /// Evaluates the given argument and terminates the program directly.
    /// The program will return the given value as its final result.
    ///
    /// Even if the argument causes an exception, it is returned directly too.
    ///
    /// If `exit` is reached via an `import`-ed module, it will stop the main program too.
    "exit"(1) => |state, args| {
        let value = args[0].eval(state).map(Cow::into_owned);
        state.exit_unwind_value = Some(value);
        Ok(Atom::Null)
    }
    /// Evaluates the given argument as a string, then treats this string as Regulus code and executes it.
    /// Returns the result of that program.
    ///
    /// Variables defined inside the evaluated code are not visible outside of the `eval` invocation.
    ///
    /// TODO: think about imports, test them
    "eval"(1) => |state, args| {
        let code = args[0].eval(state)?.string()?;
        Runner::new().code(code).run().0
    }
    /// Mark a variable identifier as global.
    ///
    /// This does not require the identifier to be defined at this time.
    "global"(1) => |state, args| {
        let var = args[0].variable("`global(1)` expects a variable argument")?;
        state.storage.global_idents.insert(var.clone());
        Ok(Atom::Null)
    }
}
