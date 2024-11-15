use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write};
use std::sync::{OnceLock, RwLock, RwLockWriteGuard};

pub(crate) fn get_mut<T>(val: &OnceLock<RwLock<T>>) -> RwLockWriteGuard<T> {
    val.get().unwrap().write().unwrap()
}

pub static STDIN: OnceLock<RwLock<Box<dyn Send + Sync + BufRead>>> = OnceLock::new();
pub static STDOUT: OnceLock<RwLock<Box<dyn Send + Sync + Write>>> = OnceLock::new();
pub static STDERR: OnceLock<RwLock<Box<dyn Send + Sync + Write>>> = OnceLock::new();

/// Register the regular stdio stream handlers.
/// Should be called before anything else if this is used.
///
/// # Panics
/// Panics if any of the streams was already set before.
pub fn set_regular() {
    set(&STDIN, Box::new(BufReader::new(stdin())));
    set(&STDOUT, Box::new(stdout()));
    set(&STDERR, Box::new(stderr()));
}

/// Set one stream to a certain value.
/// 
/// # Panics 
/// Panics if the stream was already set.
pub fn set<T>(stream: &OnceLock<RwLock<T>>, value: T) {
    stream
        .set(RwLock::new(value))
        .ok()
        .expect("stream was already set before");
}