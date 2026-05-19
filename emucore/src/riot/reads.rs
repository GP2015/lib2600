use crate::{
    common::read::{multi::MultiRead, single::SingleRead},
    riot::regs::RiotRegs,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotLineReads {
    pub db: MultiRead<8>,
    pub pa: MultiRead<8>,
    pub pb: MultiRead<5>,

    pub a: MultiRead<7>,
    pub cs1: SingleRead,
    pub cs2: SingleRead,
    pub rs: SingleRead,
    pub rw: SingleRead,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotAllReads {
    pub line: RiotLineReads,
    pub reg: RiotRegs,
}

impl RiotAllReads {
    pub const fn new(lines: RiotLineReads, regs: RiotRegs) -> Self {
        Self {
            line: lines,
            reg: regs,
        }
    }

    pub const fn update(&mut self, regs: RiotRegs) {
        self.reg = regs;
    }
}
