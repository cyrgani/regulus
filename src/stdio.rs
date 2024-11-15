use std::io::{stderr, stdin, stdout, BufRead, BufReader, Stderr, Stdout, Write};
use std::str;
use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub enum WriteHandle<T> {
    Regular(T),
    Buffer(Vec<u8>),
}

impl<T: Write> WriteHandle<T> {
    pub(crate) fn write_all(&mut self, buf: &[u8]) {
        match self {
            Self::Regular(val) => val.write_all(buf).unwrap(),
            Self::Buffer(val) => val.write_all(buf).unwrap(),
        }
    }

    /// Return a string representation of this handle if it is a buffer.
    ///
    /// # Panics
    /// Panics if it is not a buffer.
    pub fn get_buffer(&self) -> &str {
        let Self::Buffer(buf) = self else {
            unreachable!()
        };
        str::from_utf8(buf).unwrap()
    }
}

pub struct Stream<T>(OnceLock<RwLock<T>>);

impl<T> Stream<T> {
    const fn new() -> Self {
        Self(OnceLock::new())
    }

    /// Set this stream to a certain value.
    ///
    /// # Panics
    /// Panics if the stream was already set.
    pub fn set(&self, value: T) {
        self.0
            .set(RwLock::new(value))
            .ok()
            .expect("stream was already set before");
    }

    /// Returns a readable guard referencing this stream.
    ///
    /// # Panics
    /// Panics if the stream is uninitialized or poisoned.
    pub fn get(&self) -> RwLockReadGuard<T> {
        self.0.get().unwrap().read().unwrap()
    }

    pub(crate) fn get_mut(&self) -> RwLockWriteGuard<T> {
        self.0.get().unwrap().write().unwrap()
    }
}

pub static STDIN: Stream<Box<dyn Send + Sync + BufRead>> = Stream::new();
pub static STDOUT: Stream<WriteHandle<Stdout>> = Stream::new();
pub static STDERR: Stream<WriteHandle<Stderr>> = Stream::new();

/// Register the regular stdio stream handlers.
/// Should be called before anything else if this is used.
///
/// # Panics
/// Panics if any of the streams was already set before.
pub fn set_regular() {
    STDIN.set(Box::new(BufReader::new(stdin())));
    STDOUT.set(WriteHandle::Regular(stdout()));
    STDERR.set(WriteHandle::Regular(stderr()));
}
