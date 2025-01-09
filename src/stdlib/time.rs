use crate::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

export! {
    now, now_nanos_part,
}

fn epoch_duration() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("internal time error")
}

function! {
    name: now,
    argc: Some(0),
    callback: |_, _| {
        Ok(Atom::Int(epoch_duration().as_secs() as i64))
    },
}

function! {
    name: now_nanos_part,
    argc: Some(0),
    callback: |_, _| {
        Ok(Atom::Int(i64::from(epoch_duration().subsec_nanos())))
    },
}
