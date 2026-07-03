#![warn(clippy::pedantic, clippy::nursery)]

mod cpu_instr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Token, punctuated::Punctuated};

#[proc_macro]
pub fn str_pattern_from_mnemonic(input: TokenStream) -> TokenStream {
    let input_parsed: Punctuated<Ident, Token![,]> =
        syn::parse_macro_input!(input with Punctuated::parse_terminated);

    let pattern = input_parsed
        .iter()
        .flat_map(|m| cpu_instr::MNEM[&m.to_string()].iter())
        .collect::<Vec<_>>();

    quote! {
        [#(MultiRead::from_pattern(#pattern)),*]
    }
    .into()
}
