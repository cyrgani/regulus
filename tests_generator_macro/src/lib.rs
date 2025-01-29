mod make_tests;

use proc_macro::TokenStream;
use std::path::PathBuf;

#[proc_macro]
pub fn make_tests(_: TokenStream) -> TokenStream {
    make_tests::make_tests_for_dir(PathBuf::from("tests/programs"))
}
