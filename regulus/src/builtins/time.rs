use crate::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn epoch_duration() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("internal time error")
}

functions! {
    /// Returns the current time in seconds (Unix epoch) as an integer.
    ///
    /// The stable version of this function is in the `time` STL module.
    "__builtin_now"(0) => |_, _| {
        Atom::int_from_rust_int(epoch_duration().as_secs())
    }
    /// Returns the nanosecond part of the current time as an integer.
    ///
    /// The stable version of this function is in the `time` STL module.
    "__builtin_now_nanos_part"(0) => |_, _| {
        Atom::int_from_rust_int(epoch_duration().subsec_nanos())
    }
}
