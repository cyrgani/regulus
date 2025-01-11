use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::token::Paren;
use syn::{parenthesized, ExprClosure, LitInt, Token};

pub struct Functions(pub Vec<Function>);

pub struct Function {
    pub ident: TokenStream,
    pub argc_parens: Paren,
    pub argc: Option<usize>,
    pub arrow: Token![=>],
    pub body: ExprClosure,
}

impl Parse for Functions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut functions = Vec::new();
        while !input.is_empty() {
            functions.push(input.parse()?);
        }
        Ok(Self(functions))
    }
}

impl Parse for Function {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident = TokenStream::new();
        while !input.peek(Paren) {
            ident.extend(input.parse::<TokenStream>()?);
        }

        let argc_buffer;
        Ok(Self {
            ident: input.parse()?,
            argc_parens: parenthesized!(argc_buffer in input),
            argc: {
                if let Ok(int) = argc_buffer.parse::<LitInt>() { 
                    Some(int.base10_parse::<usize>()?)
                } else if input.parse::<Token![_]>().is_ok() {
                    None
                } else {
                    return Err(input.error("invalid argc"))
                }
            },
            arrow: input.parse()?,
            body: input.parse()?,
        })
    }
}

pub fn functions(input: Functions) -> TokenStream {
    let mut output = TokenStream::new();
    
    for function in input.0 {
        let ident = &function.ident;
    }
    
    output
}
