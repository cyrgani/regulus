use crate::prelude::*;

functions! {
    help(1) => |_, _| {
        Exception::new_err("help not yet implemented", Error::Unimplemented)
    }
}