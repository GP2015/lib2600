mod cycle;
mod instr;
mod raw;

use crate::build_src::cpu::instr::OP_MAP;
use std::{fs, path::Path};

pub fn generate(out_dir: &str, name: &str) {
    let path = Path::new(&out_dir).join(format!("{name}.rs"));

    dbg!(OP_MAP);

    let s = String::new();
    fs::write(&path, s).expect("file exists and write succeeds");
}
