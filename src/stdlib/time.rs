use crate::function;
use crate::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn functions() -> Vec<Function> {
    vec![now()]
}

function! {
    name: now,
    argc: Some(0),
    callback: |_, _| {
        Ok(Atom::Int(SystemTime::now().duration_since(UNIX_EPOCH).expect("internal time error").as_secs() as i64))
    },
}
