use crate::build_src::cpu::cycle::byte::ByteReg;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WordReg {
    ByteExt(ByteReg),
    Ptr,
    Addr,
    Pc,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WordModif {
    Add(isize, bool),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WordVal {
    Lit(usize),
    Reg(WordReg),
    RegModif(WordReg, WordModif),
}

impl WordVal {
    pub const fn stack() -> Self {
        Self::RegModif(WordReg::ByteExt(ByteReg::Sp), WordModif::Add(0x100, false))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WordInstr {
    reg: WordReg,
    val: WordVal,
}

impl WordInstr {
    pub const fn new(reg: WordReg, val: WordVal) -> Self {
        Self { reg, val }
    }

    pub const fn inc(reg: WordReg, allow_page_cross: bool) -> Self {
        Self::new(
            reg,
            WordVal::RegModif(reg, WordModif::Add(1, allow_page_cross)),
        )
    }

    pub const fn dec(reg: WordReg, allow_page_cross: bool) -> Self {
        Self::new(
            reg,
            WordVal::RegModif(reg, WordModif::Add(-1, allow_page_cross)),
        )
    }
}
