pub mod data;

include!(concat!(env!("OUT_DIR"), "/cpu_gen.rs"));

use crate::{
    common::bit::{multi::BitStringState, single::BitState},
    cpu::data::CpuOutput,
};

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

impl Cpu {
    pub fn new(low_pc: BitStringState<8>, high_pc: BitStringState<8>) -> Self {
        let mut pc = BitStringState::default();
        pc[0..8].copy_from_slice(low_pc.as_slice());
        pc[8..16].copy_from_slice(high_pc.as_slice());

        Self {
            a: [BitState::Unknown; _].into(),
            x: [BitState::Unknown; _].into(),
            y: [BitState::Unknown; _].into(),
            pc,
            sp: [BitState::Unknown; _].into(),
            n: BitState::Unknown,
            v: BitState::Unknown,
            b: BitState::Unknown,
            d: BitState::Unknown,
            i: BitState::High,
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
