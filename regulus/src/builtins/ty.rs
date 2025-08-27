/*
IDEA: method syntax:

type(
    Fraction,
    numerator,
    denominator,

    add(self, other, _(
        frac_add(self, other)
    ))
),

def(fictional_main, _(
    =(f, Fraction(4, 3)),
    =(g, Fraction(5, 2)),
    .(f, add(g))
))
*/

use crate::exception::{ArgumentError, NameError, SyntaxError};
use crate::prelude::*;
use std::collections::{HashMap, HashSet};

functions! {
    /// Defines a new type.
    /// The first argument must be given and is the ident of the type.
    /// All further arguments are its fields or its defaulted values.
    ///
    /// If the argument is just an identifier, it is a field.
    /// If it is a function call of the form `=(name, value)`
    /// (this must be `=` and not `assign` or another alias),
    /// this adds a field `name` which has the value `value` by default.
    /// Accordingly, this value must not and can not be set in the constructor.
    ///
    /// Methods can be added by using the defaulted value syntax as
    /// `=(method_name, fn(self, arg1, arg2, function_body()))`.
    ///
    /// TODO: In the future, it will probably be supported to use `def` directly for this purpose.
    "type"(_) => |state, args| {
        let Some((ident, fields)) = args.split_first() else {
            raise!(state, ArgumentError, "`type` takes at least one argument");
        };
        let var = ident.variable("`type` must take a variable as first argument")?;

        let mut required_fields = vec![];
        let mut defaulted_fields = vec![];
        let mut found_fields = HashSet::new();

        for field in fields {
            match field {
                Argument::Atom(..) => raise!(state, SyntaxError, "`type` field arguments should be variables or `=` calls"),
                Argument::FunctionCall(call, _) => {
                    // TODO: `def` should be allowed; aliases of `=` should be allowed too.
                    if call.name != "=" {
                        raise!(state, SyntaxError, "defaulted `type` values must use `=`");
                    }
                    let [Argument::Variable(name, _), value] = call.args.as_slice() else {
                        raise!(state, SyntaxError, "defaulted `type` values must have the form `=(name, value)`");
                    };
                    if found_fields.contains(name) {
                        raise!(state, SyntaxError, "duplicate `type` field `{name}`");
                    }
                    found_fields.insert(name);
                    defaulted_fields.push((name.clone(), value.eval(state)?.into_owned()));
                },
                Argument::Variable(name, _) => {
                    if found_fields.contains(name) {
                        raise!(state, SyntaxError, "duplicate `type` field `{name}`");
                    }
                    found_fields.insert(name);
                    required_fields.push(name.clone());
                },
            }
        }
        let ty_id = state.make_type_id();

        let function = Function::new(
            String::new(),
            Some(required_fields.len()),
            Box::new(move |state, args| {
                let mut fields = required_fields
                    .iter()
                    .zip(args)
                    .map(|(field, arg)| Ok((field.clone(), arg.eval(state)?.into_owned())))
                    .collect::<Result<HashMap<String, Atom>>>()?;
                fields.extend(defaulted_fields.clone());
                Ok(Atom::Object(Object::new(fields, ty_id)))
            }),
        );

        state.storage.insert(var, Atom::Function(function));
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
        obj.data.get(field).cloned().ok_or_else(|| state.raise(NameError, format!("object has no field named `{field}`")))
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
        *obj.data.get_mut(field).ok_or_else(|| state.raise(NameError, format!("object has no field named `{field}`")))? = value.into_owned();
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
            raise!(state, SyntaxError, "too few arguments for `@`");
        };
        let obj = obj_arg.eval(state)?.object()?;
        let method_name = method.variable("`@` expected the name of a method as second arg")?;
        let Some(func) = obj.data.get(method_name) else {
            raise!(state, NameError, "object has no method `{method_name}`");
        };
        let func = func.function()?;
        let mut args = vec![obj_arg.clone()];
        args.extend_from_slice(rest);
        func.call(state, &args, &format!("<object>.{method_name}"))
    }
    /// Returns the type id corresponding to the given value.
    ///
    /// A type id is a positive integer. Primitive types are currently represented with these IDs
    /// (note that this may change at any point):
    ///
    /// * Int: 0
    /// * Bool: 1
    /// * Null: 2
    /// * List: 3
    /// * String: 4
    /// * Function: 5
    /// * Object: 6+ (depending on the type of object).
    "type_id"(1) => |state, args| {
        Ok(Atom::Int(args[0].eval(state)?.ty_id()))
    }
}
