mod cycle;
mod instr;

use alloc::{format, string::String};
use std::{fs, path::Path};

pub fn generate(out_dir: &str, name: &str) {
    let path = Path::new(&out_dir).join(format!("{name}.rs"));

    let op_map = instr::OP_MAP;

    let s = String::new();
    fs::write(&path, s).expect("path exists and write succeeds");
}
