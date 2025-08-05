//! Builtin functions which will never have a stable equivalent and are for internal use only.

use crate::prelude::*;
use crate::state::TestHelper;
use std::fs;
use std::io::Write;

functions! {
    /// Prints the debug representation of the given argument to stdout, followed by a newline.
    ///
    /// NOTE: the output format of this method is unstable.
    /// NOTE: this method may be removed in the future.
    "__builtin_rust_debug"(1) => |state, args| {
        let arg_val = args[0].eval(state)?;
        let s = format!("{arg_val:?}\n");
        state.write_to_stdout(&s);
        Ok(Atom::Null)
    }
    /// TODO
    "__builtin_prelude_import"(0) => |_state, _| {
        Ok(Atom::Null)
    }
    /// TODO
    "__builtin_file_catch_assert_eq"(1) => |state, args| {
        file_catch_assert_eq(state, args)
    }
}

fn file_catch_assert_eq(state: &mut State, args: &[Argument]) -> Result<Atom> {
    if state.test_helper.is_none() {
        let path = state
            .current_file_path
            .as_ref()
            .unwrap()
            .with_extension("exc_stderr");

        if let Ok(content) = fs::read_to_string(&path) {
            let mut lines = vec![];
            let mut exc = String::new();
            for line in content.lines() {
                if line.is_empty() {
                    assert!(!exc.is_empty(), "too many empty lines found in exc_stderr file");
                    lines.push(exc);
                    exc = String::new();
                } else {
                    exc.push_str(line);
                    exc.push('\n');
                }
            }
            state.test_helper = Some(TestHelper::Normal {
                line_number: 0,
                expected_lines: lines,
            });
        } else {
            let file = fs::File::create(path).unwrap();
            state.test_helper = Some(TestHelper::Bless(file));
        }
    }

    let mut exc = args[0]
        .eval(state)
        .expect_err("`__builtin_file_catch_assert_eq` arg did not cause an exception")
        .to_string();

    match state.test_helper.as_mut().unwrap() {
        TestHelper::Bless(file) => {
            exc.push_str("\n\n");
            file.write_all(exc.as_bytes()).unwrap();
            Ok(Atom::Null)
        }
        TestHelper::Normal {
            line_number,
            expected_lines,
        } => {
            let expected = &expected_lines[*line_number].trim_end();
            if exc == *expected {
                *line_number += 1;
                Ok(Atom::Null)
            } else {
                Err(Exception::with_trace(
                    Error::Other("Test".to_string()),
                    format!(
                        "test did not produce expected exc_stderr: \n
expected: {expected}\n
got:      {exc}"
                    ),
                    // skip this call itself
                    &state.backtrace[0..state.backtrace.len() - 1],
                ))
            }
        }
    }
}
