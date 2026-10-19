pub mod data;

include!(concat!(env!("OUT_DIR"), "/cpu_gen.rs"));

use crate::{BitState, BitStringState, cpu::data::CpuOutput};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Cpu {
    a: BitStringState<8>,
    x: BitStringState<8>,
    y: BitStringState<8>,
    pc: BitStringState<16>,
    sp: BitStringState<8>,
    n: BitState,
    v: BitState,
    b: BitState,
    d: BitState,
    i: BitState,
    z: BitState,
    c: BitState,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: [BitState::Unknown; _].into(),
            x: [BitState::Unknown; _].into(),
            y: [BitState::Unknown; _].into(),
            pc: [BitState::Unknown; _].into(),
            sp: [BitState::Unknown; _].into(),
            n: BitState::Unknown,
            v: BitState::Unknown,
            b: BitState::Unknown,
            d: BitState::Unknown,
            i: BitState::Unknown,
            z: BitState::Unknown,
            c: BitState::Unknown,
        }
    }

    pub fn tick_rising(&mut self, rdy_read: BitState) -> CpuOutput {
        todo!()
    }

    pub fn tick_falling(&mut self, rdy_read: BitState, db_read: BitStringState<8>) {
        todo!()
    }
}
