use crate::exception::ArgumentError;
use crate::prelude::*;
use std::borrow::Cow;

#[derive(Clone)]
struct FnArgument {
    name: String,
    variadic: bool,
    lazy: bool,
}

impl FnArgument {
    fn new(name: impl Into<String>) -> Self {
        let mut name = name.into();
        let variadic = name.starts_with('[') && name.ends_with(']');
        if variadic {
            name = name
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .to_string();
        }
        let lazy = name.starts_with('$');
        if lazy {
            name = name.strip_prefix('$').unwrap().to_string();
        }
        Self {
            name,
            variadic,
            lazy,
        }
    }
}

// TODO:
//  think about the distinction between using the def-site state or the call-site state
//  for evaluating the argument
fn make_lazy(argument: Argument) -> Atom {
    Atom::Function(Function::new(
        "",
        Some(0),
        Box::new(move |state, _| {
            state.storage.current_scope -= 1;
            let v = argument.eval(state).map(Cow::into_owned);
            state.storage.current_scope += 1;
            v
        }),
    ))
}

fn define_function(body: &Argument, fn_args: &[Argument], state: &State) -> Result<Atom> {
    let body = body.clone();
    let function_arg_names = fn_args
        .iter()
        .map(|fn_arg| {
            Ok(FnArgument::new(fn_arg.variable(
                "Error during definition: invalid args were given!",
                state,
            )?))
        })
        .collect::<Result<Vec<_>>>()?;

    let (argc, min_required_args) = if let Some(variadic_idx) = function_arg_names
        .iter()
        .enumerate()
        .find_map(|(idx, arg)| if arg.variadic { Some(idx) } else { None })
    {
        if variadic_idx == function_arg_names.len() - 1 {
            (None, function_arg_names.len() - 1)
        } else {
            raise!(
                state,
                ArgumentError,
                "variadic argument must be the last of the fn arguments"
            );
        }
    } else {
        (Some(function_arg_names.len()), function_arg_names.len())
    };

    let function = Function::new(
        state.current_doc_comment.as_ref().unwrap(),
        argc,
        Box::new(move |state, args| {
            if args.len() < min_required_args && argc.is_none() {
                raise!(
                    state,
                    ArgumentError,
                    "too few arguments to variadic function: expected at least {min_required_args}, found {}",
                    args.len()
                );
            }

            // prevent arguments from overwriting each other, ex. f(a,b) calls f(b,a)
            let mut arg_values = Vec::with_capacity(args.len());

            // TODO:
            //  see `lazy_functions.re`, `variadic_functions.re`,
            //  `lazy_and_variadic_functions.re` tests for more TODOs
            for (idx, signature_arg) in function_arg_names.iter().enumerate() {
                if signature_arg.variadic {
                    let mut va_list = Vec::with_capacity(args.len() - idx);
                    for arg in args.iter().skip(idx) {
                        va_list.push(if signature_arg.lazy {
                            make_lazy(arg.clone())
                        } else {
                            arg.eval(state)?.into_owned()
                        });
                    }
                    arg_values.push((signature_arg.clone(), Atom::List(va_list)));
                } else {
                    let arg_result = if signature_arg.lazy {
                        make_lazy(args[idx].clone())
                    } else {
                        args[idx].eval(state)?.into_owned()
                    };
                    arg_values.push((signature_arg.clone(), arg_result));
                }
            }

            // a function call should have its own scope and not leak variables
            // except for globals
            state.storage.start_scope();
            for (name, value) in arg_values {
                if name.lazy {
                    state.storage.current_scope -= 1;
                }
                state.storage.insert(name.name, value);
                if name.lazy {
                    state.storage.current_scope += 1;
                }
            }

            let function_result = body.eval(state).map(Cow::into_owned);
            state.storage.end_scope();

            function_result
        }),
    );

    Ok(Atom::Function(function))
}

functions! {
    /// Defines a new function.
    /// The first argument is the function identifier and the last argument is the function body.
    /// All arguments in between are the names of the function arguments that can be accessed in
    /// the function body.
    /// Values defined in the function are scoped and cannot be accessed outside of the function body.
    "def"(_) => |state, args| {
        let [var, fn_args @ .., body] = args else {
            raise!(
                state,
                ArgumentError,
                "too few arguments passed to `def`: expected at least 2, found {}", args.len()
            );
        };
        let var = var.variable("Error during function definition: no valid variable was given to define to!", state)?;

        state.storage.insert(var, define_function(body, fn_args, state)?);
        Ok(Atom::Null)
    }
    /// Creates a new function and returns it.
    ///
    /// The last argument is the function body.
    /// All arguments before are the names of the function arguments that can be accessed in
    /// the function body.
    /// Values defined in the function are scoped and cannot be accessed outside of the function body.
    "fn"(_) => |state, args| {
        let Some((body, fn_args)) = args.split_last() else {
            raise!(state, ArgumentError, "`fn` invocation is missing body");
        };
        define_function(body, fn_args, state)
    }
}
