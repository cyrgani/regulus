use crate::Directory;
use crate::builtins::all_functions;
use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Stderr, Stdout, Write, stderr, stdin, stdout};
use std::path::Path;
use std::{io, str};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct TaggedIdent {
    pub ident: String,
    pub source: Source,
}

impl TaggedIdent {
    pub fn regular(ident: impl AsRef<str>, layer: u16) -> Self {
        Self {
            ident: ident.as_ref().to_string(),
            source: Source::Regular { layer },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Source {
    Regular {
        /// roughly similar to one stack frame per layer, 0 is the main scope
        layer: u16,
    },
    Import,
}

impl Source {
    const fn layer(self) -> Option<u16> {
        match self {
            Self::Regular { layer } => Some(layer),
            Self::Import => None,
        }
    }
}

pub struct Storage {
    pub data: HashMap<TaggedIdent, Atom>,
    pub global_idents: HashSet<String>,
    pub current_layer: u16,
}

impl Storage {
    pub fn initial() -> Self {
        Self {
            data: all_functions()
                .into_iter()
                .map(|(ident, function)| (TaggedIdent::regular(ident, 0), function))
                .collect(),
            global_idents: HashSet::new(),
            current_layer: 0,
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Atom> {
        let candidates = self
            .data
            .iter()
            .filter_map(|(ident, val)| {
                if ident.ident == name.as_ref() {
                    if true {
                        //let Source::Regular { .. } = ident.source {
                        Some((ident.source, val))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        candidates
            .into_iter()
            .max_by_key(|(source, _)| source.layer())
            .map(|(_, val)| val)
    }

    pub fn remove_entry(&mut self, name: impl AsRef<str>) -> Option<(TaggedIdent, Atom)> {
        let ident = self
            .data
            .keys()
            .find(|ident| ident.ident == name.as_ref())?
            .clone();
        self.data.remove_entry(&ident)
    }

    pub fn insert(&mut self, name: impl AsRef<str>, value: Atom) {
        let layer = if self.global_idents.contains(name.as_ref()) {
            0
        } else {
            self.current_layer
        };

        self.data.insert(
            TaggedIdent {
                ident: name.as_ref().to_string(),
                source: Source::Regular { layer },
            },
            value,
        );
    }

    pub fn insert_from_import(&mut self, name: impl AsRef<str>, value: Atom) {
        self.data.insert(
            TaggedIdent {
                ident: name.as_ref().to_string(),
                source: Source::Import,
            },
            value,
        );
    }

    pub fn global_items(&self) -> impl Iterator<Item = (TaggedIdent, Atom)> {
        self.data
            .iter()
            .filter(|(ident, _)| self.global_idents.contains(&ident.ident))
            .map(|(ident, atom)| (ident.clone(), atom.clone()))
    }

    pub fn add_global(&mut self, name: impl AsRef<str>, value: Atom) {
        self.global_idents.insert(name.as_ref().to_owned());
        self.data.insert(TaggedIdent::regular(name, 0), value);
    }

    pub fn remove_top_layer(&mut self) {
        assert!(self.current_layer > 0);
        let layer = self.current_layer;
        self.data
            .retain(|ident, _| ident.source != Source::Regular { layer });
        self.current_layer -= 1;
    }

    /// everything that is kept after `import`
    pub fn nonlocals(&self) -> impl Iterator<Item = (String, Atom)> {
        self.data
            .iter()
            .filter_map(|(ident, atom)| match ident.source {
                Source::Regular { .. } => Some((ident.ident.clone(), atom.clone())),
                Source::Import => None,
            })
    }
}

// TODO: users should be able to set their own stderr/out/in streams too
pub struct State {
    pub storage: Storage,
    stdin: Box<dyn BufRead>,
    stdout: WriteHandle<Stdout>,
    stderr: WriteHandle<Stderr>,
    pub(crate) file_directory: Directory,
    pub(crate) exit_unwind_value: Option<Result<Atom>>,
}

impl State {
    pub fn initial(current_dir: impl AsRef<Path>) -> Self {
        Self::initial_with_dir(Directory::Regular(current_dir.as_ref().to_path_buf()))
    }

    pub(crate) fn initial_with_dir(current_dir: Directory) -> Self {
        Self {
            storage: Storage::initial(),
            stdin: Box::new(BufReader::new(stdin())),
            stdout: WriteHandle::Regular(stdout()),
            stderr: WriteHandle::Regular(stderr()),
            file_directory: current_dir,
            exit_unwind_value: None,
        }
    }

    pub fn get_function(&self, name: &str) -> Result<Function> {
        match self.storage.get(name) {
            Some(atom) => {
                if let Atom::Function(func) = atom {
                    Ok(func.clone())
                } else {
                    raise!(Error::Name, "`{name}` is not a function!")
                }
            }
            None => raise!(Error::Name, "No function `{name}` found!"),
        }
    }

    pub(crate) fn stdin(&mut self) -> &mut dyn BufRead {
        &mut self.stdin
    }

    pub(crate) fn stdout(&mut self) -> &mut dyn Write {
        match &mut self.stdout {
            WriteHandle::Buffer(buf) => buf,
            WriteHandle::Regular(stdout) => stdout,
        }
    }

    #[expect(dead_code, reason = "nothing outputs to stderr yet")]
    pub(crate) fn stderr(&mut self) -> &mut dyn Write {
        match &mut self.stderr {
            WriteHandle::Buffer(buf) => buf,
            WriteHandle::Regular(stderr) => stderr,
        }
    }

    #[doc(hidden)]
    pub fn testing_setup(dir_path: &str, stdin: &str) -> Self {
        Self {
            storage: Storage::initial(),
            stdin: Box::new(BufReader::new(VecReader(stdin.as_bytes().to_vec()))),
            stdout: WriteHandle::Buffer(vec![]),
            stderr: WriteHandle::Buffer(vec![]),
            file_directory: Directory::Regular(dir_path.into()),
            exit_unwind_value: None,
        }
    }

    #[doc(hidden)]
    pub fn testing_read_stdout(&self) -> &str {
        self.stdout.read_buffer()
    }

    #[doc(hidden)]
    pub fn testing_read_stderr(&self) -> &str {
        self.stderr.read_buffer()
    }
}

struct VecReader(Vec<u8>);
impl Read for VecReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.as_slice().read(buf)
    }
}

enum WriteHandle<T> {
    Regular(T),
    Buffer(Vec<u8>),
}

impl<T> WriteHandle<T> {
    /// Return a string representation of this handle if it is a buffer.
    ///
    /// # Panics
    /// Panics if it is not a buffer or if it does not contain valid UTF-8.
    fn read_buffer(&self) -> &str {
        let Self::Buffer(buf) = self else {
            unreachable!()
        };
        str::from_utf8(buf).unwrap()
    }
}
