use crate::build::cpu::spec::{
    instr::{AddrMode, InstrStem},
    opcode::OPCODE_TABLE,
};
use arrayvec::ArrayVec;
use std::array;
use strum::EnumCount;

type AddrMapEntry = ArrayVec<AddrMode, { AddrMode::COUNT }>;

pub fn generate() -> [AddrMapEntry; InstrStem::COUNT] {
    let mut map = array::from_fn(|_| ArrayVec::new());

    for instr in OPCODE_TABLE {
        let entry = &mut map[instr.stem as usize];
        if !entry.contains(&instr.addr) {
            entry.push(instr.addr);
        }
    }

    map
}
