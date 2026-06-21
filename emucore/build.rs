use proc_macro2::Span;
use quote::quote;
use serde::Deserialize;
use std::{env, fs, path::Path};
use syn::{File as SynFile, Ident};

#[derive(Debug, Deserialize)]
struct CpuStructVariant {
    name: String,
    pattern: String,
}

#[derive(Debug, Deserialize)]
struct CpuStruct {
    name: String,
    variants: Vec<CpuStructVariant>,
}

#[derive(Debug, Deserialize)]
struct Config {
    structs: Vec<CpuStruct>,
}

fn main() {
    let file_str = fs::read_to_string("src/build/cpu_instr.toml").unwrap();
    let cpu_structs: Config = toml::from_str(&file_str).unwrap();

    let cpu_structs_tokens = cpu_structs.structs.iter().map(|cpu_struct| {
        let struct_name_ident = Ident::new(&cpu_struct.name, Span::call_site());
        let cpu_variants_tokens = cpu_struct.variants.iter().map(|var| {
            let name = Ident::new(&var.name, Span::call_site());
            quote! { #name }
        });

        quote! {
            #[derive(Clone, Copy, Debug)]
            pub enum #struct_name_ident {
                #(#cpu_variants_tokens),*
            }
        }
    });

    let token_stream = quote! { #(#cpu_structs_tokens)* };

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("cpu_instr.rs");

    let syntax_tree: SynFile = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    fs::write(dest, formatted).unwrap();
}
