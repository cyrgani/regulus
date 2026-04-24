use crate::Atom;
use crate::builtins::all_functions;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub(crate) enum StoredValue {
    Global(Atom),
    /// An identifier may refer to any number of atoms within different scopes.
    /// Only the innermost one will be considered, until its scope ends.
    Locals(Vec<(usize, Atom)>),
}

impl StoredValue {
    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Self::Global(a) => Some(a),
            Self::Locals(v) => v.last().map(|(_, a)| a),
        }
    }

    pub fn update(&mut self, atom: Atom, scope: usize) {
        match self {
            Self::Global(a) => *a = atom,
            Self::Locals(vec) => {
                if let Some(last) = vec.last_mut()
                    && last.0 == scope
                {
                    last.1 = atom;
                } else {
                    vec.push((scope, atom));
                }
            }
        }
    }

    pub fn reduce_by_scope(&mut self, scope: usize) {
        if let Self::Locals(vec) = self
            && let Some(last) = vec.last()
            && last.0 == scope
        {
            let _ = vec.pop();
        }
    }
}

// TODO: consider merging this type with `State`
pub struct Storage {
    pub(crate) data: HashMap<String, StoredValue>,
    pub(crate) current_scope: usize,
}

#[expect(
    clippy::missing_const_for_fn,
    reason = "type is not constructible in const anyway"
)]
impl Storage {
    pub fn initial() -> Self {
        Self {
            data: all_functions()
                .into_iter()
                .map(|(name, f)| (name, StoredValue::Locals(vec![(0, f)])))
                .collect(),
            current_scope: 0,
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Atom> {
        self.data.get(name.as_ref())?.as_atom()
    }

    pub fn insert(&mut self, name: impl AsRef<str>, value: Atom) {
        match self.data.entry(name.as_ref().to_string()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().update(value, self.current_scope);
            }
            Entry::Vacant(entry) => {
                entry.insert(StoredValue::Locals(vec![(self.current_scope, value)]));
            }
        }
    }

    pub fn add_global(&mut self, name: impl AsRef<str>, value: Atom) {
        self.data
            .insert(name.as_ref().to_string(), StoredValue::Global(value));
    }

    pub(crate) fn start_scope(&mut self) {
        self.current_scope += 1;
    }

    pub(crate) fn end_scope(&mut self) {
        // TODO: consider removing dead locals (with an empty vec) from the storage
        for val in self.data.values_mut() {
            val.reduce_by_scope(self.current_scope);
        }
        self.current_scope -= 1;
    }

    #[expect(clippy::missing_panics_doc)]
    pub fn extend_from(&mut self, other: Self) {
        assert_eq!(other.current_scope, 0);
        for (name, value) in other.data {
            match value {
                StoredValue::Global(global) => {
                    self.add_global(name, global);
                }
                StoredValue::Locals(mut locals) => {
                    assert_eq!(locals.len(), 1);
                    self.insert(name, locals.pop().unwrap().1);
                }
            }
        }
    }

    pub fn undefine(&mut self, name: impl AsRef<str>) -> Option<Atom> {
        match self.data.remove(name.as_ref())? {
            StoredValue::Global(global) => Some(global),
            StoredValue::Locals(mut locals) => locals.pop().map(|(_, a)| a),
        }
    }

    pub fn all_data(&self) -> impl Iterator<Item = (String, Atom)> {
        self.data
            .iter()
            .filter_map(|(ident, value)| Some((ident.clone(), value.as_atom()?.clone())))
    }

    pub fn all_globals(&self) -> impl Iterator<Item = (String, Atom)> {
        self.data.iter().filter_map(|(ident, value)| {
            if let StoredValue::Global(atom) = value {
                Some((ident.clone(), atom.clone()))
            } else {
                None
            }
        })
    }
}
