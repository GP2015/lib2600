mod cpu_instr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Token, punctuated::Punctuated};

#[proc_macro]
pub fn cpu_instr_mnemonic(input: TokenStream) -> TokenStream {
    let input_parsed: Punctuated<Ident, Token![,]> =
        syn::parse_macro_input!(input with Punctuated::parse_terminated);

    let pattern = input_parsed
        .iter()
        .map(|m| cpu_instr::INSTR_MNEMONICS[&m.to_string()].clone())
        .collect::<Vec<_>>()
        .join(",");

    quote! { #pattern }.into()
}
