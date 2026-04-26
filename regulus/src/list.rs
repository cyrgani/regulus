use crate::atom::Atom;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct List(pub Rc<Vec<Atom>>);

impl List {
    pub fn new(v: Vec<Atom>) -> Self {
        Self(Rc::new(v))
    }

    pub fn make_mut(&mut self) -> &mut Vec<Atom> {
        Rc::make_mut(&mut self.0)
    }
}

impl Deref for List {
    type Target = Vec<Atom>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
