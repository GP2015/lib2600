use crate::common::{BitReg, MBitReg};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CpuRegs {
    pub instr_cycle: MBitReg<3>,
    pub a: MBitReg<8>,
    pub x: MBitReg<8>,
    pub y: MBitReg<8>,
    pub pc: MBitReg<16>,
    pub s: MBitReg<8>,
    pub n: BitReg,
    pub v: BitReg,
    pub b: BitReg,
    pub d: BitReg,
    pub i: BitReg,
    pub z: BitReg,
    pub c: BitReg,
}

impl CpuRegs {
    pub fn new() -> Self {
        Self {
            instr_cycle: [BitReg::Unknown; _].into(),
            a: [BitReg::Unknown; _].into(),
            x: [BitReg::Unknown; _].into(),
            y: [BitReg::Unknown; _].into(),
            pc: [BitReg::Unknown; _].into(),
            s: [BitReg::Unknown; _].into(),
            n: BitReg::Unknown,
            v: BitReg::Unknown,
            b: BitReg::Unknown,
            d: BitReg::Unknown,
            i: BitReg::Unknown,
            z: BitReg::Unknown,
            c: BitReg::Unknown,
        }
    }
}
