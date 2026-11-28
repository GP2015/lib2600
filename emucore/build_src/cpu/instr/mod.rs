mod raw;

use crate::build_src::cpu::instr::raw::{RawAddrMode, RawInstr, RawStem};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InstrReg {
    Acc,
    X,
    Y,
    Sp,
    Sr,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Flag {
    N,
    V,
    D,
    I,
    Z,
    C,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImplOp {
    Asl,
    Lsr,
    Rol,
    Ror,
    Clear(Flag),
    Set(Flag),
    Inc(InstrReg),
    Dec(InstrReg),
    Transfer(InstrReg, InstrReg),
    Nop,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GenReadOp {
    And,
    Ora,
    Eor,
    Adc,
    Sbc,
    Bit,
    Load(InstrReg),
    Compare(InstrReg),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GenRmwOp {
    Asl,
    Lsr,
    Rol,
    Ror,
    Inc,
    Dec,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GenOp {
    Read(GenReadOp),
    Rmw(GenRmwOp),
    Store(InstrReg),
}

impl GenOp {
    const fn from_raw(raw: RawInstr) -> Self {
        match raw.stem {
            RawStem::And => Self::Read(GenReadOp::And),
            RawStem::Ora => Self::Read(GenReadOp::Ora),
            RawStem::Eor => Self::Read(GenReadOp::Eor),
            RawStem::Adc => Self::Read(GenReadOp::Adc),
            RawStem::Sbc => Self::Read(GenReadOp::Sbc),
            RawStem::Bit => Self::Read(GenReadOp::Bit),
            RawStem::Lda => Self::Read(GenReadOp::Load(InstrReg::Acc)),
            RawStem::Ldx => Self::Read(GenReadOp::Load(InstrReg::X)),
            RawStem::Ldy => Self::Read(GenReadOp::Load(InstrReg::Y)),
            RawStem::Cmp => Self::Read(GenReadOp::Compare(InstrReg::Acc)),
            RawStem::Cpx => Self::Read(GenReadOp::Compare(InstrReg::X)),
            RawStem::Cpy => Self::Read(GenReadOp::Compare(InstrReg::Y)),

            RawStem::Asl => Self::Rmw(GenRmwOp::Asl),
            RawStem::Lsr => Self::Rmw(GenRmwOp::Lsr),
            RawStem::Rol => Self::Rmw(GenRmwOp::Rol),
            RawStem::Ror => Self::Rmw(GenRmwOp::Ror),
            RawStem::Inc => Self::Rmw(GenRmwOp::Inc),
            RawStem::Dec => Self::Rmw(GenRmwOp::Dec),

            RawStem::Sta => Self::Store(InstrReg::Acc),
            RawStem::Stx => Self::Store(InstrReg::X),
            RawStem::Sty => Self::Store(InstrReg::Y),

            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Instr {
    Brk,
    Rti,
    Rts,
    Jsr,
    JmpInd,
    JmpAbs,
    BranchOnSet(Flag),
    BranchOnClear(Flag),
    StackPush(InstrReg),
    StackPull(InstrReg),
    Impl(ImplOp),
    Imm(GenReadOp),
    Abs(GenOp),
    Zpg(GenOp),
    AbsIdx(InstrReg, GenOp),
    ZpgIdx(InstrReg, GenOp),
    XInd(GenOp),
    IndY(GenOp),
    Illegal,
}

impl Instr {
    pub const fn from_raw(raw: RawInstr) -> Self {
        match raw.stem {
            RawStem::Brk => Self::Brk,
            RawStem::Rti => Self::Rti,
            RawStem::Rts => Self::Rts,
            RawStem::Jsr => Self::Jsr,

            RawStem::Jmp => match raw.addr {
                RawAddrMode::Abs => Self::JmpAbs,
                RawAddrMode::Ind => Self::JmpInd,
                _ => unreachable!(),
            },

            RawStem::Pha => Self::StackPush(InstrReg::Acc),
            RawStem::Php => Self::StackPush(InstrReg::Sr),
            RawStem::Pla => Self::StackPull(InstrReg::Acc),
            RawStem::Plp => Self::StackPull(InstrReg::Sr),

            RawStem::Alr
            | RawStem::Anc
            | RawStem::Ane
            | RawStem::Arr
            | RawStem::Dcp
            | RawStem::Isc
            | RawStem::Las
            | RawStem::Lax
            | RawStem::Lxa
            | RawStem::Rla
            | RawStem::Rra
            | RawStem::Sax
            | RawStem::Sbx
            | RawStem::Sha
            | RawStem::Shx
            | RawStem::Shy
            | RawStem::Slo
            | RawStem::Sre
            | RawStem::Tas
            | RawStem::Jam => Self::Illegal,

            RawStem::Nop if !matches!(raw.addr, RawAddrMode::Impl) => Self::Illegal,

            _ => Self::from_non_unique_raw(raw),
        }
    }

    const fn from_non_unique_raw(raw: RawInstr) -> Self {
        match raw.addr {
            RawAddrMode::Ind => unreachable!(),

            RawAddrMode::Rel => match raw.stem {
                RawStem::Bpl => Self::BranchOnClear(Flag::N),
                RawStem::Bvc => Self::BranchOnClear(Flag::V),
                RawStem::Bne => Self::BranchOnClear(Flag::Z),
                RawStem::Bcc => Self::BranchOnClear(Flag::C),
                RawStem::Bmi => Self::BranchOnSet(Flag::N),
                RawStem::Bvs => Self::BranchOnSet(Flag::V),
                RawStem::Beq => Self::BranchOnSet(Flag::Z),
                RawStem::Bcs => Self::BranchOnSet(Flag::C),
                _ => unreachable!(),
            },

            RawAddrMode::Impl => Self::Impl(match raw.stem {
                RawStem::Asl => ImplOp::Asl,
                RawStem::Lsr => ImplOp::Lsr,
                RawStem::Rol => ImplOp::Rol,
                RawStem::Ror => ImplOp::Ror,
                RawStem::Clc => ImplOp::Clear(Flag::C),
                RawStem::Cld => ImplOp::Clear(Flag::D),
                RawStem::Cli => ImplOp::Clear(Flag::I),
                RawStem::Clv => ImplOp::Clear(Flag::V),
                RawStem::Dex => ImplOp::Dec(InstrReg::X),
                RawStem::Dey => ImplOp::Dec(InstrReg::Y),
                RawStem::Inx => ImplOp::Inc(InstrReg::X),
                RawStem::Iny => ImplOp::Inc(InstrReg::Y),
                RawStem::Nop => ImplOp::Nop,
                RawStem::Sec => ImplOp::Set(Flag::C),
                RawStem::Sed => ImplOp::Set(Flag::D),
                RawStem::Sei => ImplOp::Set(Flag::I),
                RawStem::Tax => ImplOp::Transfer(InstrReg::Acc, InstrReg::X),
                RawStem::Tay => ImplOp::Transfer(InstrReg::Acc, InstrReg::Y),
                RawStem::Tsx => ImplOp::Transfer(InstrReg::Sp, InstrReg::X),
                RawStem::Txa => ImplOp::Transfer(InstrReg::X, InstrReg::Acc),
                RawStem::Txs => ImplOp::Transfer(InstrReg::X, InstrReg::Sp),
                RawStem::Tya => ImplOp::Transfer(InstrReg::Y, InstrReg::Acc),
                _ => unreachable!(),
            }),

            _ => Self::from_gen_op_raw(raw),
        }
    }

    const fn from_gen_op_raw(raw: RawInstr) -> Self {
        let gen_op = GenOp::from_raw(raw);

        match raw.addr {
            RawAddrMode::Imm => {
                let GenOp::Read(gen_read_op) = gen_op else {
                    unreachable!()
                };

                Self::Imm(gen_read_op)
            }

            RawAddrMode::Abs => Self::Abs(gen_op),
            RawAddrMode::Zpg => Self::Zpg(gen_op),
            RawAddrMode::XInd => Self::XInd(gen_op),
            RawAddrMode::IndY => Self::IndY(gen_op),
            RawAddrMode::AbsX => Self::AbsIdx(InstrReg::X, gen_op),
            RawAddrMode::AbsY => Self::AbsIdx(InstrReg::Y, gen_op),
            RawAddrMode::ZpgX => Self::ZpgIdx(InstrReg::X, gen_op),
            RawAddrMode::ZpgY => Self::ZpgIdx(InstrReg::Y, gen_op),

            _ => unreachable!(),
        }
    }
}

pub const OP_MAP: [Instr; 256] = {
    let mut instrs = [Instr::Illegal; _];

    let mut i = 0;
    while i < 256 {
        instrs[i] = Instr::from_raw(raw::RAW_OP_MAP[i]);
        i += 1;
    }

    instrs
};
