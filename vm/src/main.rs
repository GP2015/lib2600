#![warn(clippy::pedantic, clippy::nursery)]

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

use anyhow::anyhow;
use std::io::Read;

struct Cpu {
    a: Option<u8>,
    x: Option<u8>,
    y: Option<u8>,
    pc: u16,
    s: Option<u8>,
    n: Option<bool>,
    v: Option<bool>,
    b: Option<bool>,
    d: Option<bool>,
    i: Option<bool>,
    z: Option<bool>,
    c: Option<bool>,
}

impl Cpu {
    pub fn new(rom: &[u8]) -> Self {
        Self {
            a: None,
            x: None,
            y: None,
            pc: (u16::from(rom[0xFFD]) << 8) | u16::from(rom[0xFFC]),
            s: None,
            n: None,
            v: None,
            b: None,
            d: None,
            i: None,
            z: None,
            c: None,
        }
    }

    pub fn tick(&mut self, rom: &[u8], ram: &mut [Option<u8>]) -> anyhow::Result<()> {
        let opcode = if (self.pc >> 12) & 1 == 0 {
            ram[(self.pc & 0xFFF) as usize]
                .ok_or_else(|| anyhow!("attempted to access uninitialised RAM"))?
        } else {
            rom[(self.pc & 0xFFF) as usize]
        };

        let instr = OPCODE_LUT[usize::from(opcode)];

        todo!()
    }
}

fn main() -> anyhow::Result<()> {
    let Some(path) = std::env::args().nth(1) else {
        return Err(anyhow!("no path provided"));
    };

    let mut rom = [0; 4096];
    std::fs::File::open(path)?.read_exact(&mut rom)?;

    let mut ram = [None; 4096];

    let mut cpu = Cpu::new(&rom);

    loop {
        cpu.tick(&rom, &mut ram)?;
    }
}
