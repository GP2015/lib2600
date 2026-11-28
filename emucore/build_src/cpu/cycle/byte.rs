use crate::build_src::cpu::instr::InstrReg;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ByteReg {
    Hold0,
    Hold1,
    Hold2,
    Db,
    Pch,
    Pcl,
    Sp,
    Acc,
    X,
    Y,
    Sr,
}

impl From<InstrReg> for ByteReg {
    fn from(instr_reg: InstrReg) -> Self {
        match instr_reg {
            InstrReg::Acc => Self::Acc,
            InstrReg::X => Self::X,
            InstrReg::Y => Self::Y,
            InstrReg::Sp => Self::Sp,
            InstrReg::Sr => Self::Sr,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ByteModif {
    Add(isize),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ByteVal {
    Lit(usize),
    Reg(ByteReg),
    RegModif(ByteReg, ByteModif),
    SrWithB(bool),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ByteInstr {
    reg: ByteReg,
    val: ByteVal,
}

impl ByteInstr {
    pub const fn new(reg: ByteReg, val: ByteVal) -> Self {
        Self { reg, val }
    }

    pub const fn inc(reg: ByteReg) -> Self {
        Self::new(reg, ByteVal::RegModif(reg, ByteModif::Add(1)))
    }

    pub const fn dec(reg: ByteReg) -> Self {
        Self::new(reg, ByteVal::RegModif(reg, ByteModif::Add(-1)))
    }
}
