use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Argument {
    FunctionCall(FunctionCall),
    Atom(Atom),
    Variable(String),
}

impl Argument {
    pub fn eval(&self, program: &[Argument], storage: &mut Storage) -> ProgResult<Atom> {
        match self {
            Argument::FunctionCall(call) => call.eval(program, storage),
            Argument::Atom(atom) => Ok(atom.clone()),
            Argument::Variable(var) => match storage.get(var) {
                Some(value) => Ok(value.clone()),
                None => Err(Exception {
                    msg: format!("No variable named `{var}` found!"),
                    error: Error::Name,
                }),
            },
        }
    }
}
