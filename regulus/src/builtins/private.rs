//! Builtin functions which will never have a stable equivalent and are for internal use only.

use crate::prelude::*;
use crate::state::TestHelper;
use std::fs;
use std::io::Write;

functions! {
    /// TODO
    "__builtin_prelude_import"(0) => |_state, _| {
        Ok(Atom::Null)
    }
    /// Evaluates the given argument, checks that it causes an exception and compares
    /// the exception and backtrace to the fitting `.exc_stderr` file.
    /// Not meant to be used outside of tests.
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
