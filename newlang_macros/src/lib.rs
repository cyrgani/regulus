mod functions;

use crate::functions::Functions;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn functions(input: TokenStream) -> TokenStream {
    functions::functions(parse_macro_input!(input as Functions)).into()
}
