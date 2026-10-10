use crate::{
    common::read::{multi::MultiRead, single::SingleRead},
    riot::regs::RiotRegs,
};

type RiotRegReads = RiotRegs;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotLineReads {
    pub a: MultiRead<7>,
    pub db: MultiRead<8>,
    pub pa: MultiRead<8>,
    pub pb: MultiRead<5>,
    pub cs1: SingleRead,
    pub cs2: SingleRead,
    pub rs: SingleRead,
    pub rw: SingleRead,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotAllReads {
    pub line: RiotLineReads,
    pub reg: RiotRegReads,
}

impl RiotAllReads {
    pub const fn new(lines: RiotLineReads, regs: RiotRegReads) -> Self {
        Self {
            line: lines,
            reg: regs,
        }
    }

    pub const fn update(&mut self, regs: RiotRegReads) {
        self.reg = regs;
    }
}
