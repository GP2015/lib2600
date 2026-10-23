#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]

mod build_src;

use std::env;

pub fn main() {
    println!("cargo::rerun-if-changed=build_src/");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is defined");
    build_src::run(&out_dir);
}
