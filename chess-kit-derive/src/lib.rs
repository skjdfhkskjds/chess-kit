use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod bitops;
mod arithmetic;

#[proc_macro_derive(BitOps)]
pub fn derive_bitops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match bitops::expand_bitops(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(Arithmetic)]
pub fn derive_arithmetic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match arithmetic::expand_arithmetic(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
