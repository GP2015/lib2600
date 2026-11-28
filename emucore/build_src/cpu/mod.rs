mod cycle;
mod instr;

use crate::build_src::cpu::cycle::InstrCycles;
use alloc::{format, string::String};
use std::{fs, path::Path};

pub fn generate(out_dir: &str, name: &str) {
    let path = Path::new(&out_dir).join(format!("{name}.rs"));

    let cycles = instr::OP_MAP.map(InstrCycles::from_instr);

    let s = String::new();
    fs::write(&path, s).expect("path exists and write succeeds");
}
