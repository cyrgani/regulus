use crate::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn epoch_duration() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("internal time error")
}

functions! {
    now(0) => |_, _| {
        Ok(Atom::Int(epoch_duration().as_secs() as i64))
    }
    now_nanos_part(0) => |_, _| {
        Ok(Atom::Int(i64::from(epoch_duration().subsec_nanos())))
    }
}
