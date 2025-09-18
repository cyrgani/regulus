use crate::exception::ImportError;
use crate::interned_stdlib::INTERNED_STL;
use crate::prelude::*;
use crate::state::Directory;
use std::fs;
use std::path::{Path, PathBuf};

fn import(state: &mut State, args: &[Argument]) -> Result<Atom> {
    let name = args[0].variable(
        "`import` argument must be a variable, string syntax was removed",
        state,
    )?;
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        raise!(
            state,
            ImportError,
            "invalid characters in import name `{name}`, only a-Z, 0-9 and _ are allowed",
        );
    }

    // lookup order:
    // 1. look inside the programs current directory
    // 2. look in the global stl directory
    let mut import_state = State::new();
    import_state.import_stack.clone_from(&state.import_stack);
    let mut found = false;
    if let Directory::Regular(dir_path) = &state.file_directory
        && let Some(path) = try_resolve_import_in_dir(name, dir_path)
    {
        if import_state.import_stack.contains(&path) {
            raise!(
                state,
                ImportError,
                "cyclic import of `{name}` at path `{}` detected",
                path.display()
            );
        }
        import_state = import_state.with_source_file(&path).unwrap();
        import_state.import_stack.push(path);
        found = true;
    }

    if !found && let Some(code) = try_resolve_import_in_stl(name) {
        import_state = import_state.with_code(code);
        import_state.set_current_file_path(format!("<stl:{name}>"));
        found = true;
    }
    if !found {
        raise!(
            state,
            ImportError,
            "failed to find file for importing `{name}`"
        );
    }

    let atom = import_state.run();

    if let Some(exit_unwind_value) = import_state.exit_unwind_value {
        state.exit_unwind_value = Some(exit_unwind_value);
        return Ok(Atom::Null);
    }
    let atom = atom?;
    state.storage.extend_from(import_state.storage);

    Ok(atom)
}

/// Returns:
/// * `None` if the resolution in the given directory failed,
/// * `Some(path)` if the code was found at `path` in the given directory.
fn try_resolve_import_in_dir(name: &str, dir_path: &Path) -> Option<PathBuf> {
    let paths = fs::read_dir(dir_path)
        .unwrap_or_else(|err| {
            panic!(
                "error when reading directory `{}`: {err}",
                dir_path.display()
            )
        })
        .flatten();
    for item in paths {
        if *item.file_name() == *format!("{name}.{FILE_EXTENSION}") {
            return Some(item.path());
        }
    }
    None
}

fn try_resolve_import_in_stl(name: &str) -> Option<String> {
    INTERNED_STL.get(name).map(ToString::to_string)
}

functions! {
    /// Imports a file, either from the stl or the local directory.
    /// TODO document the exact algorithm and hierarchy more clearly, also the return value of this function
    "import"(1) => |state, args| {
        import(state, args)
    }
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
        import_state.run()?;

        state.storage.extend_from(import_state.storage);
        Ok(Atom::Null)
    }
}
