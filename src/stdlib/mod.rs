use crate::prelude::Function;

pub mod cast;
pub mod core;
pub mod io;
pub mod list;
pub mod logic;
pub mod math;
pub mod string;
pub mod time;

pub type NamedFunction = (&'static str, Function);
