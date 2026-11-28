mod bit;
mod byte;
mod word;

use crate::build_src::cpu::{
    cycle::{
        bit::{BitInstr, BitReg, BitVal},
        byte::{ByteInstr, ByteReg, ByteVal},
        word::{WordInstr, WordModif, WordReg, WordVal},
    },
    instr::Instr,
};
use alloc::{vec, vec::Vec};
use derive_more::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SubInstr {
    Bit(BitInstr),
    Byte(ByteInstr),
    Word(WordInstr),
}

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct Cycle([Vec<SubInstr>; 2]);

impl Cycle {
    fn write(addr_val: WordVal, db_val: ByteVal) -> Self {
        Self([
            vec![
                SubInstr::Word(WordInstr::new(WordReg::Addr, addr_val)),
                SubInstr::Byte(ByteInstr::new(ByteReg::Db, db_val)),
                SubInstr::Bit(BitInstr::new(BitReg::Rw, BitVal::Lit(false))),
            ],
            Vec::new(),
        ])
    }

    fn read(addr_val: WordVal, db_to: Option<ByteReg>) -> Self {
        Self([
            vec![
                SubInstr::Word(WordInstr::new(WordReg::Addr, addr_val)),
                SubInstr::Bit(BitInstr::new(BitReg::Rw, BitVal::Lit(true))),
            ],
            db_to.map_or_else(Vec::new, |reg| {
                vec![SubInstr::Byte(ByteInstr::new(
                    reg,
                    ByteVal::Reg(ByteReg::Db),
                ))]
            }),
        ])
    }

    fn with_bit(mut self, instr: BitInstr) -> Self {
        self[1].push(SubInstr::Bit(instr));
        self
    }

    fn with_byte(mut self, instr: ByteInstr) -> Self {
        self[1].push(SubInstr::Byte(instr));
        self
    }

    fn with_word(mut self, instr: WordInstr) -> Self {
        self[1].push(SubInstr::Word(instr));
        self
    }

    fn read_pc(db_to: Option<ByteReg>, inc_pc: bool) -> Self {
        let mut cycle = Self::read(WordVal::Reg(WordReg::Pc), db_to);
        if inc_pc {
            cycle[1].push(SubInstr::Word(WordInstr::inc(WordReg::Pc, true)));
        }
        cycle
    }

    fn stack_push(val: ByteVal, dec_sp: bool) -> Self {
        let mut cycle = Self::write(WordVal::stack(), val);
        if dec_sp {
            cycle[1].push(SubInstr::Byte(ByteInstr::dec(ByteReg::Sp)));
        }
        cycle
    }

    fn stack_pull(reg: ByteReg, inc_sp: bool) -> Self {
        let mut cycle = Self::read(WordVal::stack(), Some(reg));
        if inc_sp {
            cycle[1].push(SubInstr::Byte(ByteInstr::inc(ByteReg::Sp)));
        }
        cycle
    }
}

// Note that this doesn't include the initial cycle,
// which is the same for every instruction.
#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct InstrCycles(Vec<Cycle>);

impl InstrCycles {
    pub fn from_instr(instr: Instr) -> Self {
        Self(match instr {
            Instr::Brk => vec![
                Cycle::read_pc(None, true),
                Cycle::stack_push(ByteVal::Reg(ByteReg::Pch), true),
                Cycle::stack_push(ByteVal::Reg(ByteReg::Pcl), true),
                Cycle::stack_push(ByteVal::SrWithB(true), true),
                Cycle::read(WordVal::Lit(0xFFFE), Some(ByteReg::Pcl)),
                Cycle::read(WordVal::Lit(0xFFFF), Some(ByteReg::Pch)),
            ],
            Instr::Rti => vec![
                Cycle::read_pc(None, true),
                Cycle::read(WordVal::stack(), None).with_byte(ByteInstr::inc(ByteReg::Sp)),
                Cycle::stack_pull(ByteReg::Sr, true),
                Cycle::stack_pull(ByteReg::Pcl, true),
                Cycle::stack_pull(ByteReg::Pch, false),
            ],
            Instr::Rts => vec![
                Cycle::read_pc(None, true),
                Cycle::read(WordVal::stack(), None).with_byte(ByteInstr::inc(ByteReg::Sp)),
                Cycle::stack_pull(ByteReg::Pcl, true),
                Cycle::stack_pull(ByteReg::Pch, false),
                Cycle::read_pc(None, true),
            ],
            Instr::StackPush(instr_reg) => vec![
                Cycle::read_pc(None, true),
                Cycle::stack_push(ByteVal::Reg(instr_reg.into()), true),
            ],
            Instr::StackPull(instr_reg) => vec![
                Cycle::read_pc(None, true),
                Cycle::read(WordVal::stack(), None).with_byte(ByteInstr::inc(ByteReg::Sp)),
                Cycle::stack_pull(instr_reg.into(), false),
            ],
            Instr::Jsr => vec![
                Cycle::read_pc(Some(ByteReg::Hold0), true),
                Cycle::read(WordVal::stack(), None),
                Cycle::stack_push(ByteVal::Reg(ByteReg::Pch), true),
                Cycle::stack_push(ByteVal::Reg(ByteReg::Pcl), true),
                Cycle::read_pc(Some(ByteReg::Pch), false)
                    .with_byte(ByteInstr::new(ByteReg::Pcl, ByteVal::Reg(ByteReg::Hold0))),
            ],
            Instr::JmpAbs => vec![
                Cycle::read_pc(Some(ByteReg::Hold0), true),
                Cycle::read_pc(Some(ByteReg::Pch), false)
                    .with_byte(ByteInstr::new(ByteReg::Pcl, ByteVal::Reg(ByteReg::Hold0))),
            ],
            Instr::JmpInd => vec![
                Cycle::read_pc(Some(ByteReg::Hold0), true),
                Cycle::read_pc(Some(ByteReg::Hold1), true),
                Cycle::read(WordVal::Reg(WordReg::Ptr), Some(ByteReg::Hold2)),
                Cycle::read(
                    WordVal::RegModif(WordReg::Ptr, WordModif::Add(1, false)),
                    Some(ByteReg::Hold2),
                ),
            ],
            Instr::BranchOnSet(flag) => vec![],
            Instr::BranchOnClear(flag) => vec![],
            Instr::Impl(impl_op) => vec![],
            Instr::Imm(gen_read_op) => vec![],
            Instr::Abs(gen_op) => vec![],
            Instr::Zpg(gen_op) => vec![],
            Instr::AbsIdx(reg, gen_op) => vec![],
            Instr::ZpgIdx(reg, gen_op) => vec![],
            Instr::XInd(gen_op) => vec![],
            Instr::IndY(gen_op) => vec![],
            Instr::Illegal => vec![],
        })
    }
}
