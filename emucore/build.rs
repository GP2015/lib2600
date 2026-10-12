#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]

mod build {
    pub mod cpu;
}

use std::env;

pub fn main() {
    println!("cargo::rerun-if-changed=build/");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is defined");
    build::cpu::generate(&out_dir, "cpu_gen");
}
