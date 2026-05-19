use crate::{
    common::read::{multi::MultiRead, single::SingleRead},
    cpu::regs::CpuRegs,
};

type CpuRegReads = CpuRegs;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CpuLineReads {
    pub db: MultiRead<8>,
    pub rdy: SingleRead,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CpuAllReads {
    pub line: CpuLineReads,
    pub reg: CpuRegReads,
}

impl CpuAllReads {
    pub const fn new(lines: CpuLineReads, regs: CpuRegReads) -> Self {
        Self {
            line: lines,
            reg: regs,
        }
    }

    pub const fn update(&mut self, regs: CpuRegReads) {
        self.reg = regs;
    }
}
