//! Builtin functions which will never have a stable equivalent and are for internal use only.

use crate::interned_stdlib::INTERNED_STL;
use crate::prelude::*;
use crate::state::Directory;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn epoch_duration() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("internal time error")
}

functions! {
    /// Imports the prelude from the STL.
    /// This is implicitly done on startup.
    /// Calling this function manually is not supported.
    "__builtin_prelude_import"(0) => |state, _| {
        if matches!(state.file_directory, Directory::InternedSTL) {
            return Ok(Atom::Null);
        }
        let name = "prelude";
        let mut import_state = State::new();
        let code = INTERNED_STL[name];
        import_state = import_state.with_code(code);
        import_state.set_current_file_path(format!("<stl:{name}>"));

        import_state.storage.global_idents.clone_from(&state.storage.global_idents);
        import_state.storage.data.extend(state.storage.global_items());
        import_state.run()?;

        for (k, v) in import_state.storage.data {
            state.storage.insert(k, v);
        }
        state.storage.global_idents = import_state.storage.global_idents;
        Ok(Atom::Null)
    }
    /// Evaluates the given argument, extracts the exception and prints it to stderr.
    /// Not meant to be used outside of tests.
    "__builtin_print_catch"(1) => |state, args| {
        let exc = args[0].eval(state).expect_err("`__builtin_print_catch` arg should cause exception");
        state.write_to_stderr(&exc.to_string());
        state.write_to_stderr("\n");
        Ok(Atom::Null)
    }
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
    /// Evaluates both arguments and returns whether they are equal.
    /// TODO: define this behavior in edge cases and document it (in the STL, on `==` and `!=`)
    "__builtin_atom_eq"(2) => |state, args| {
        Ok(Atom::Bool(args[0].eval(state)?.into_owned() == *args[1].eval(state)?))
    }
}
