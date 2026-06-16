use itertools::izip;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::{array, env, fs, path::Path};
use syn::Ident;

fn make_segment_lut(path: &str, enum_name: &str, token_stream: &mut TokenStream2) -> [Ident; 256] {
    let parsed_file: Vec<Vec<String>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_owned()).collect())
        .collect();

    let enum_ident = Ident::new(enum_name, Span::call_site());

    let variants = parsed_file.iter().map(|line| {
        let variant = Ident::new(&line[0], Span::call_site());
        quote! { #variant }
    });

    token_stream.extend(quote! {
        #[derive(Clone, Copy, Debug)]
        enum #enum_ident {
            #(#variants),*
        }
    });

    let mut lut: [Option<Ident>; 256] = array::from_fn(|_| None);

    for row in parsed_file {
        for pattern in &row[1..] {
            for opcode in iter_possible_vals(pattern) {
                let term_ident = Ident::new(&row[0], Span::call_site());
                if lut[opcode] != Some(term_ident.clone()) {
                    assert!(lut[opcode].is_none());
                    lut[opcode] = Some(term_ident);
                }
            }
        }
    }

    lut.map(Option::unwrap)
}

fn iter_possible_vals(rep: &str) -> impl Iterator<Item = usize> {
    let stripped_rep = rep.replace('_', "");
    assert!(stripped_rep.len() == 8);

    let mut unknown_indices: Vec<_> = Vec::new();
    let mut mask = 0;

    for (i, c) in stripped_rep.chars().enumerate() {
        match c {
            '0' => mask |= 0 << i,
            '1' => mask |= 1 << i,
            '?' => unknown_indices.push(i),
            _ => unreachable!(),
        }
    }

    (0..(1 << unknown_indices.len())).map(move |id| {
        let mut val = mask;

        for (src_bit, &dst_bit) in unknown_indices.iter().enumerate() {
            val |= ((id >> src_bit) & 1) << dst_bit;
        }

        val
    })
}

fn main() {
    let mut token_stream = TokenStream2::new();

    macro_rules! make_segment_lut {
        ($(($lut_name:ident, $enum_name:ident, $file_name:literal)),+ $(,)?) => {$(
            let path = concat!("src/instr/", $file_name);
            let enum_name_str = stringify!($enum_name);
            let $lut_name = make_segment_lut(path, enum_name_str, &mut token_stream);
        )+};
    }

    make_segment_lut!(
        (base_instr_lut, BaseInstruction, "base_instr.txt"),
        (addr_mode_lut, AddrMode, "addr_mode.txt"),
        (addr_idx_lut, AddrModeIdx, "addr_idx.txt")
    );

    let instrs = izip!(
        base_instr_lut.iter(),
        addr_mode_lut.iter(),
        addr_idx_lut.iter()
    )
    .map(|(base_instr, addr_mode, addr_idx)| {
        quote! {
            Instruction {
                base_instr: BaseInstruction::#base_instr,
                addr_mode: AddrMode::#addr_mode,
                addr_idx: AddrModeIdx::#addr_idx,
            }
        }
    });

    token_stream.extend(quote! {
        #[derive(Clone, Copy, Debug)]
        struct Instruction {
            base_instr: BaseInstruction,
            addr_mode: AddrMode,
            addr_idx: AddrModeIdx,
        }

        const OPCODE_LUT: [Instruction; 256] = [#(#instrs),*];
    });

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("lut.rs");
    fs::write(dest, token_stream.to_string()).unwrap();
}
