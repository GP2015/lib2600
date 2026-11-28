#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BitReg {
    Rw,
    N,
    V,
    D,
    I,
    Z,
    C,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BitVal {
    Lit(bool),
    Reg(BitReg),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BitInstr {
    reg: BitReg,
    val: BitVal,
}

impl BitInstr {
    pub const fn new(reg: BitReg, val: BitVal) -> Self {
        Self { reg, val }
    }
}
