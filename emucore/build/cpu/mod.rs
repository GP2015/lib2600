mod instr_dec;
mod raw_instr;

use crate::build::cpu::instr_dec::OP_MAP;
use std::{fs, path::Path};

pub fn generate(out_dir: &str, name: &str) {
    let path = Path::new(&out_dir).join(format!("{name}.rs"));

    let tmp = &OP_MAP;

    let s = String::new();
    fs::write(&path, s).expect("file exists and write succeeds");
}
