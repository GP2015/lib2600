#![warn(clippy::pedantic, clippy::nursery)]

mod cpu_instr;

use crate::cpu_instr::Lut;
use proc_macro::TokenStream;
use quote::quote;
use std::sync::LazyLock;
use syn::{Ident, Token, parse_macro_input, punctuated::Punctuated};

fn pat(input: TokenStream, lut: &LazyLock<Lut>) -> TokenStream {
    let input_parsed: Punctuated<Ident, Token![,]> =
        parse_macro_input!(input with Punctuated::parse_terminated);

    let patterns = input_parsed
        .iter()
        .flat_map(|m| lut[m.to_string().as_str()].iter())
        .map(|&s| {
            let bits = s.chars().map(|c| {
                let enum_var = match c {
                    '0' => quote!(Low),
                    '1' => quote!(High),
                    '?' => quote!(Unknown),
                    _ => unreachable!(),
                };

                quote!(SingleRead::#enum_var)
            });

            quote!([#(#bits),*].into())
        });

    quote!([#(#patterns),*]).into()
}

macro_rules! create_pats {
    ($(($fn_name:ident, $lut:path)),+ $(,)?) => {$(
        #[proc_macro]
        pub fn $fn_name(input: TokenStream) -> TokenStream {
            pat(input, &$lut)
        }
    )+};
}

create_pats!(
    (mnem_pat, cpu_instr::MNEM),
    (addr_mode_pat, cpu_instr::ADDR_MODE),
    (addr_mode_idx_pat, cpu_instr::ADDR_MODE_IDX),
);
