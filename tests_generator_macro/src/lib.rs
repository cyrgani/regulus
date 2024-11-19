mod make_tests;

use proc_macro::TokenStream;

#[proc_macro]
pub fn make_tests(_: TokenStream) -> TokenStream {
    make_tests::make_tests_for_dir("./tests/programs")
}
