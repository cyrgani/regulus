use crate::prelude::*;

enum Void {}
/// Type alias used so that opt passes can use `?` to exit easily without returning anything meaningful.
type Unit = Option<Void>;

#[expect(dead_code)]
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

    const fn var(&mut self) -> Option<&mut String> {
        match self {
            Self::Variable(a, _) => Some(a),
            _ => None,
        }
    }
}

struct OptData {}

pub fn run_optimizations(program: &mut Argument) {
    optimize(program, &mut OptData {});
}

#[expect(clippy::only_used_in_recursion)]
fn optimize(program: &mut Argument, data: &mut OptData) -> Unit {
    inline_trivial_underscore_call(program);

    let call = program.call()?;
    for arg in &mut call.args {
        optimize(arg, data);
    }
    None
}

fn inline_trivial_underscore_call(call_arg: &mut Argument) -> Unit {
    let call = call_arg.call()?;
    if call.name == "_" {
        if call.args.is_empty() {
            *call_arg = Argument::Atom(Atom::Null, call_arg.span().clone());
        } else if call.args.len() == 1 {
            *call_arg = call.args[0].clone();
        }
    }
    None
}
