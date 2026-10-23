mod cpu;

pub fn run(out_dir: &str) {
    cpu::generate(out_dir, "cpu_gen");
}
