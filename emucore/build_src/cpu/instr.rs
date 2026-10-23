use crate::build_src::cpu::raw::{RAW_OP_MAP, RawAddrMode, RawInstr, RawStem};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Reg {
    A,
    X,
    Y,
    S,
    P,
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
    Inc(Reg),
    Dec(Reg),
    Transfer(Reg, Reg),
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
    Load(Reg),
    Compare(Reg),
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
    Store(Reg),
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
            RawStem::Lda => Self::Read(GenReadOp::Load(Reg::A)),
            RawStem::Ldx => Self::Read(GenReadOp::Load(Reg::X)),
            RawStem::Ldy => Self::Read(GenReadOp::Load(Reg::Y)),
            RawStem::Cmp => Self::Read(GenReadOp::Compare(Reg::A)),
            RawStem::Cpx => Self::Read(GenReadOp::Compare(Reg::X)),
            RawStem::Cpy => Self::Read(GenReadOp::Compare(Reg::Y)),

            RawStem::Asl => Self::Rmw(GenRmwOp::Asl),
            RawStem::Lsr => Self::Rmw(GenRmwOp::Lsr),
            RawStem::Rol => Self::Rmw(GenRmwOp::Rol),
            RawStem::Ror => Self::Rmw(GenRmwOp::Ror),
            RawStem::Inc => Self::Rmw(GenRmwOp::Inc),
            RawStem::Dec => Self::Rmw(GenRmwOp::Dec),

            RawStem::Sta => Self::Store(Reg::A),
            RawStem::Stx => Self::Store(Reg::X),
            RawStem::Sty => Self::Store(Reg::Y),

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
    StackPush(Reg),
    StackPull(Reg),
    Impl(ImplOp),
    Imm(GenReadOp),
    Abs(GenOp),
    Zpg(GenOp),
    AbsIdx(Reg, GenOp),
    ZpgIdx(Reg, GenOp),
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

            RawStem::Pha => Self::StackPush(Reg::A),
            RawStem::Php => Self::StackPush(Reg::P),
            RawStem::Pla => Self::StackPull(Reg::A),
            RawStem::Plp => Self::StackPull(Reg::P),

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
                RawStem::Dex => ImplOp::Dec(Reg::X),
                RawStem::Dey => ImplOp::Dec(Reg::Y),
                RawStem::Inx => ImplOp::Inc(Reg::X),
                RawStem::Iny => ImplOp::Inc(Reg::Y),
                RawStem::Nop => ImplOp::Nop,
                RawStem::Sec => ImplOp::Set(Flag::C),
                RawStem::Sed => ImplOp::Set(Flag::D),
                RawStem::Sei => ImplOp::Set(Flag::I),
                RawStem::Tax => ImplOp::Transfer(Reg::A, Reg::X),
                RawStem::Tay => ImplOp::Transfer(Reg::A, Reg::Y),
                RawStem::Tsx => ImplOp::Transfer(Reg::S, Reg::X),
                RawStem::Txa => ImplOp::Transfer(Reg::X, Reg::A),
                RawStem::Txs => ImplOp::Transfer(Reg::X, Reg::S),
                RawStem::Tya => ImplOp::Transfer(Reg::Y, Reg::A),
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
            RawAddrMode::AbsX => Self::AbsIdx(Reg::X, gen_op),
            RawAddrMode::AbsY => Self::AbsIdx(Reg::Y, gen_op),
            RawAddrMode::ZpgX => Self::ZpgIdx(Reg::X, gen_op),
            RawAddrMode::ZpgY => Self::ZpgIdx(Reg::Y, gen_op),

            _ => unreachable!(),
        }
    }
}

pub const OP_MAP: [Instr; 256] = {
    let mut instrs = [Instr::Illegal; _];

    let mut i = 0;
    while i < 256 {
        instrs[i] = Instr::from_raw(RAW_OP_MAP[i]);
        i += 1;
    }

    instrs
};
