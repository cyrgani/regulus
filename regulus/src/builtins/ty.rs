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

use crate::prelude::*;

functions! {
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

        let function = Function::new(
            String::new(),
            Some(fields.len()),
            Box::new(move |state, args| {
                Ok(Atom::Object(
                    fields
                        .iter()
                        .zip(args)
                        .map(|(field, arg)| Ok((field.clone(), arg.eval(state)?.into_owned())))
                        .collect::<Result<_>>()?,
                ))
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
}