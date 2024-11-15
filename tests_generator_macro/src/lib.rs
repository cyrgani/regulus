use proc_macro::TokenStream;
use std::fs::read_dir;
use std::str::FromStr;

#[proc_macro]
pub fn make_tests(_: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    for file in read_dir("./programs").unwrap() {
        let file = file.unwrap();
        if file.file_type().unwrap().is_file() {
            let name = file.file_name().into_string().unwrap();
            if let Some(name) = name.strip_suffix(".prog") {
                output.extend(
                    TokenStream::from_str(&format!(
                        r##"
#[test]
fn {name}() {{
    utils::run_test("{name}");
}}
"##,
                    ))
                    .unwrap(),
                );
            }
        }
    }

    output
}
