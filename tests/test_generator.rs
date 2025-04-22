use proc_macro::TokenStream;
use std::fs::read_dir;
use std::path::PathBuf;
use std::str::FromStr;

#[proc_macro]
pub fn make_tests(_: TokenStream) -> TokenStream {
    make_tests_for_dir(PathBuf::from("tests/programs"))
}

fn make_tests_for_dir(dir_path: PathBuf) -> TokenStream {
    let mut output = TokenStream::new();
    for entry in read_dir(&dir_path).unwrap() {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            let name = entry.file_name().into_string().unwrap();
            if let Some(name) = name.strip_suffix(".re") {
                output.extend(
                    TokenStream::from_str(&format!(
                        r##"
#[test]
fn {name}() {{
    run_test("{}", "{name}");
}}
"##,
                        dir_path.components().skip(1).collect::<PathBuf>().display()
                    ))
                    .unwrap(),
                );
            }
        } else if file_type.is_dir() {
            output.extend(make_tests_for_dir(entry.path()))
        }
    }

    output
}
