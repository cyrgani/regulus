use crate::builtins::fn_def::define_function;
use crate::prelude::*;
use std::collections::HashMap;

fn type_(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let Some((ident, fields)) = args.split_first() else {
        raise!(state, "Argument", "`type` takes at least one argument");
    };
    let var = ident.variable("`type` must take a variable as first argument", state)?;

    let mut required_fields = vec![];

    for field in fields {
        let Argument::Variable(name, _) = field else {
            raise!(
                state,
                "Syntax",
                "`type` field arguments should be variables"
            )
        };
        if required_fields.contains(name) {
            raise!(state, "Syntax", "duplicate `type` field `{name}`");
        }
        required_fields.push(name.clone());
    }
    let ty_id = state.make_type_id();

    let function = Function::new(
        String::new(),
        Some(required_fields.len()),
        move |state, args| {
            let fields = required_fields
                .iter()
                .zip(args)
                .map(|(field, arg)| Ok((field.clone(), arg.eval(state)?)))
                .collect::<Result<HashMap<String, Atom>>>()?;
            Ok(Atom::Object(Object::new(fields, ty_id)))
        },
    );

    state.storage.insert(var, Atom::Function(function));
    Ok(Atom::Null)
}

fn impl_(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let [type_ident, field, fn_args @ .., body] = args else {
        raise!(state, "Argument", "too few arguments passed to `impl`");
    };
    let type_ident =
        type_ident.variable("`impl` takes a type identifier as first argument", state)?;
    let field = field.variable("`impl` takes a field identifier as second argument", state)?;

    let func = define_function(body, fn_args, state)?;
    let old_ctor = state.get_function(type_ident)?;
    let new_ctor = old_ctor.wrap_ctor(field, func);
    state.storage.insert(type_ident, Atom::Function(new_ctor));

    Ok(Atom::Null)
}

fn default_value(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let type_ident = args[0].variable(
        "`default_value` takes a type identifier as first argument",
        state,
    )?;
    let field = args[1].variable(
        "`default_value` takes a field identifier as second argument",
        state,
    )?;
    let value = args[2].eval(state)?;
    let old_ctor = state.get_function(type_ident)?;
    let new_ctor = old_ctor.wrap_ctor(field, value);
    state.storage.insert(type_ident, Atom::Function(new_ctor));

    Ok(Atom::Null)
}

functions! {
    /// Defines a new type.
    /// The first argument must be given and is the ident of the type.
    /// All further arguments are its fields, given as identifiers.
    "type"(_) => type_
    /// Get the value of a field of an object.
    ///
    /// The first argument is the object, the second is its name as a variable.
    ///
    /// If the field does not exist on the object, an exception is raised.
    ///
    /// This function has an alias: `getattr`.
    "."(2) => |state, args| {
        let obj = args[0].eval_object(state)?;
        let field = args[1].variable("`.` takes a field identifier as second argument", state)?;
        obj.data.get(field).cloned().ok_or_else(|| state.raise("Name", format!("object has no field named `{field}`")))
    }
    /// Set the value of a field of an object to a new value and returns the updated object.
    ///
    /// The first argument is the object, the second is its name as a variable and the third is the new value.
    ///
    /// If the field does not exist on the object, an exception is raised.
    /// TODO: think if it should be allowed to add fields with this that did not exist before or not
    ///
    /// This function has an alias: `setattr`.
    "->"(3) => |state, args| {
        let mut obj = args[0].eval_object(state)?;
        let field = args[1].variable("`.` takes a field identifier as second argument", state)?;
        let value = args[2].eval(state)?;
        *obj.data_mut().get_mut(field).ok_or_else(|| {
            state.raise("Name", format!("object has no field named `{field}`"))
        })? = value;
        Ok(Atom::Object(obj))
    }
    /// Calls a method on an object with the given arguments.
    /// The object itself is implicitly added as the first argument to the method.
    ///
    /// The first argument is the object, the second the identifier of the method and all further arguments are the arguments to the method.
    ///
    /// This method has an alias: `call_method`.
    "@"(_) => |state, args| {
        let [obj_arg, method, rest @ ..] = args else {
            raise!(state, "Syntax", "too few arguments for `@`");
        };
        let obj = obj_arg.eval_object(state)?;
        let method_name = method.variable("`@` expected the name of a method as second arg", state)?;
        let Some(func_atom) = obj.data.get(method_name) else {
            raise!(state, "Name", "object has no method `{method_name}`");
        };
        let Atom::Function(func) = func_atom else {
            raise!(state, "Type", "{func_atom} is not a function");
        };
        let mut args = vec![obj_arg.clone()];
        args.extend_from_slice(rest);
        state.current_fn_name = Some(format!("<object>.{method_name}"));
        state.current_doc_comment = Some(String::new());
        func.call(state, &args)
    }
    /// Returns the type id corresponding to the given value.
    ///
    /// A type id is a positive integer. Each primitive type has a distinct ID
    /// (note that its value may change in future versions):
    /// All objects have arbitrary type ids that are larger than any of the primitive type IDs.
    ///
    /// To access them, use the constants in the `type_id.re` STL module
    /// and the `is_*` (`int`, `bool`, ...) family of functions in that module.
    "type_id"(1) => |state, args| {
        Ok(Atom::Int(args[0].eval(state)?.ty_id()))
    }
    /// Set a field of a type to a default value.
    /// Arguments: type name, field name, value.
    "default_value"(3) => default_value
    /// Defines a method on the given type with similar syntax to `def`.
    /// Arguments: type name, method name, any number of method args (usually starting with `self`), method body.
    "impl"(_) => impl_
}
