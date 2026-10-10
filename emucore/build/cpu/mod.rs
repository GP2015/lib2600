mod instrs;
mod opcodes;

use std::{fs, path::Path};

pub fn generate(out_dir: &str, name: &str) {
    let path = Path::new(&out_dir).join(format!("{name}.rs"));

    let s = String::new();
    fs::write(&path, s).expect("file exists and write succeeds");
}
