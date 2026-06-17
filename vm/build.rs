use proc_macro2::Span;
use quote::quote;
use serde::Deserialize;
use std::{array, env, fs, path::Path};
use syn::{File as SynFile, Ident};

#[derive(Debug, Deserialize)]
struct OpcodePattern(String);

impl OpcodePattern {
    fn possible_opcodes(&self) -> impl Iterator<Item = usize> {
        self.0.split(",").flat_map(|pattern| {
            assert!(pattern.len() == 8);

            let mut unknown_indices: Vec<_> = Vec::new();
            let mut mask = 0;

            for (i, c) in pattern.chars().rev().enumerate() {
                match c {
                    '0' => (),
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
        })
    }
}

#[derive(Debug, Deserialize)]
struct InstructionSegment {
    name: String,
    pattern: OpcodePattern,
}

#[derive(Debug, Deserialize)]
struct RawInstructions {
    base_instructions: Vec<InstructionSegment>,
    addressing_modes: Vec<InstructionSegment>,
    addressing_mode_registers: Vec<InstructionSegment>,
}

#[derive(Debug, Default)]
struct LutEntry {
    instr: Option<usize>,
    addr_mode: Option<usize>,
    addr_mode_reg: Option<usize>,
}

#[derive(Clone, Debug)]
enum AddrModeCount {
    None,
    Single(usize),
    Multiple,
}

impl AddrModeCount {
    fn add(&mut self, new_mode_idx: usize) {
        match self {
            Self::None => *self = Self::Single(new_mode_idx),
            Self::Single(old_mode_idx) if *old_mode_idx != new_mode_idx => *self = Self::Multiple,
            _ => (),
        }
    }
}

fn main() {
    let file_str = fs::read_to_string("src/instr.toml").unwrap();
    let raw_instrs: RawInstructions = toml::from_str(&file_str).unwrap();

    let mut lut: [LutEntry; 256] = array::from_fn(|_| LutEntry::default());

    for (i, base_instr) in raw_instrs.base_instructions.iter().enumerate() {
        for opcode in base_instr.pattern.possible_opcodes() {
            assert!(lut[opcode].instr.is_none() || lut[opcode].instr == Some(i));
            lut[opcode].instr = Some(i);
        }
    }

    let mut base_instr_map: Vec<AddrModeCount> =
        vec![AddrModeCount::None; raw_instrs.base_instructions.len()];

    for (i, addr_mode) in raw_instrs.addressing_modes.iter().enumerate() {
        for opcode in addr_mode.pattern.possible_opcodes() {
            assert!(lut[opcode].addr_mode.is_none() || lut[opcode].addr_mode == Some(i));
            lut[opcode].addr_mode = Some(i);
            base_instr_map[lut[opcode].instr.unwrap()].add(i);
        }
    }

    let mut addr_mode_map: Vec<bool> = vec![false; raw_instrs.addressing_modes.len()];

    for (i, addr_mode_reg) in raw_instrs.addressing_mode_registers.iter().enumerate() {
        for opcode in addr_mode_reg.pattern.possible_opcodes() {
            assert!(lut[opcode].addr_mode_reg.is_none() || lut[opcode].addr_mode_reg == Some(i));
            lut[opcode].addr_mode_reg = Some(i);
            addr_mode_map[lut[opcode].addr_mode.unwrap()] = true;
        }
    }

    let instr_variant_tokens = raw_instrs
        .base_instructions
        .iter()
        .enumerate()
        .map(|(i, instr)| {
            let base_instr_ident = Ident::new(&instr.name, Span::call_site());
            if matches!(base_instr_map[i], AddrModeCount::Multiple) {
                quote! { #base_instr_ident (AddressingMode) }
            } else {
                quote! { #base_instr_ident }
            }
        });

    let addr_mode_variant_tokens =
        raw_instrs
            .addressing_modes
            .iter()
            .enumerate()
            .map(|(i, addr_mode)| {
                let addr_mode_ident = Ident::new(&addr_mode.name, Span::call_site());
                if addr_mode_map[i] {
                    quote! { #addr_mode_ident (AddressingModeRegister) }
                } else {
                    quote! { #addr_mode_ident }
                }
            });

    let addr_mode_reg_variant_tokens = raw_instrs
        .addressing_mode_registers
        .iter()
        .map(|addr_mode_idx| Ident::new(&addr_mode_idx.name, Span::call_site()));

    let lut_entry_tokens = lut.iter().map(|entry| {
        let instr_idx = entry.instr.unwrap();
        let instr_ident = Ident::new(
            &raw_instrs.base_instructions[instr_idx].name,
            Span::call_site(),
        );

        if matches!(base_instr_map[instr_idx], AddrModeCount::Multiple) {
            let addr_mode_idx = entry.addr_mode.unwrap();
            let addr_mode_ident = Ident::new(
                &raw_instrs.addressing_modes[addr_mode_idx].name,
                Span::call_site(),
            );

            println!(
                "{:?}, {}, {}",
                entry,
                raw_instrs.base_instructions[instr_idx].name,
                raw_instrs.addressing_modes[addr_mode_idx].name,
            );

            if addr_mode_map[addr_mode_idx] {
                let addr_mode_reg_idx = entry.addr_mode_reg.unwrap();
                let addr_mode_reg_ident = Ident::new(
                    &raw_instrs.addressing_mode_registers[addr_mode_reg_idx].name,
                    Span::call_site(),
                );

                quote! {
                    Instruction::#instr_ident (
                        AddressingMode::#addr_mode_ident (
                            AddressingModeRegister::#addr_mode_reg_ident
                        )
                    )
                }
            } else {
                quote! {
                    Instruction::#instr_ident (AddressingMode::#addr_mode_ident)
                }
            }
        } else {
            println!(
                "{:?}, {}",
                entry, raw_instrs.base_instructions[instr_idx].name,
            );

            quote! { Instruction::#instr_ident }
        }
    });

    let token_stream = quote! {
        #[derive(Clone, Copy, Debug)]
        enum Instruction {
            #(#instr_variant_tokens),*
        }

        #[derive(Clone, Copy, Debug)]
        enum AddressingMode {
            #(#addr_mode_variant_tokens),*
        }

        #[derive(Clone, Copy, Debug)]
        enum AddressingModeRegister {
            #(#addr_mode_reg_variant_tokens),*
        }

        const OPCODE_LUT: [Instruction; 256] = [
            #(#lut_entry_tokens),*
        ];
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("gen.rs");

    let syntax_tree: SynFile = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    fs::write(dest, formatted).unwrap();
}
