use crate::prelude::*;

impl Argument {
    const fn atom(&mut self) -> Option<&mut Atom> {
        match self {
            Self::Atom(a, _) => Some(a),
            _ => None,
        }
    }

    const fn call(&mut self) -> Option<&mut FunctionCall> {
        match self {
            Self::FunctionCall(a, _) => Some(a),
            _ => None,
        }
    }
}

pub fn optimize(program: &mut Argument) -> Option<()> {
    inline_trivial_underscore_call(program);

    let call = program.call()?;
    for arg in &mut call.args {
        optimize(arg);
    }
    Some(())
}

fn inline_trivial_underscore_call(call_arg: &mut Argument) -> Option<()> {
    let call = call_arg.call()?;
    if call.name == "_" {
        if call.args.is_empty() {
            *call_arg = Argument::Atom(Atom::Null, call_arg.span().clone());
        } else if call.args.len() == 1 {
            *call_arg = call.args[0].clone();
        }
    }
    Some(())
}
