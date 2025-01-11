use proc_macro::TokenStream;
use std::fs::read_dir;
use std::path::Path;
use std::str::FromStr;

pub fn make_tests_for_dir(dir_path: impl AsRef<Path>) -> TokenStream {
    let mut output = TokenStream::new();
    for file in read_dir(&dir_path).unwrap() {
        let file = file.unwrap();
        let file_type = file.file_type().unwrap();
        if file_type.is_file() {
            let name = file.file_name().into_string().unwrap();
            if let Some(name) = name.strip_suffix(".prog") {
                output.extend(
                    TokenStream::from_str(&format!(
                        r##"
#[test]
fn {name}() {{
    run_test("{}", "{name}");
}}
"##,
                        dir_path.as_ref().display()
                    ))
                    .unwrap(),
                );
            }
        } else if file_type.is_dir() {
            output.extend(make_tests_for_dir(file.path()))
        }
    }

    output
}
