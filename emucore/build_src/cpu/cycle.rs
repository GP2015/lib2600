use crate::build_src::cpu::instr::Instr;
use alloc::{vec, vec::Vec};
use derive_more::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Loc {
    StemHold,
    Op0Hold,
    InstrCycles,

    Phi2,
    Addr,
    Db,
    Rw,

    Acc,
    X,
    Y,
    Pc,
    Sp,
    N,
    V,
    B,
    D,
    I,
    Z,
    C,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum CopyFrom {
    Lit(usize),
    Loc(Loc),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Modif {
    None,
    Inc,
    Add(usize),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct SubInstr {
    loc: Loc,
    from: CopyFrom,
    modif: Modif,
}

impl SubInstr {
    const fn new(loc: Loc, from: CopyFrom, modif: Modif) -> Self {
        Self { loc, from, modif }
    }
}

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
struct Cycle([Vec<SubInstr>; 2]);

impl Cycle {
    fn write(addr_from: CopyFrom, db_from: CopyFrom) -> Self {
        Self([
            vec![
                SubInstr::new(Loc::Addr, addr_from, Modif::None),
                SubInstr::new(Loc::Db, db_from, Modif::None),
                SubInstr::new(Loc::Rw, CopyFrom::Lit(0), Modif::None),
            ],
            Vec::new(),
        ])
    }

    fn read(addr_from: CopyFrom, db_to: Option<Loc>) -> Self {
        Self([
            vec![
                SubInstr::new(Loc::Addr, addr_from, Modif::None),
                SubInstr::new(Loc::Rw, CopyFrom::Lit(1), Modif::None),
            ],
            db_to.map_or_else(Vec::new, |loc| {
                vec![SubInstr::new(loc, CopyFrom::Loc(Loc::Db), Modif::None)]
            }),
        ])
    }

    fn read_pc(db_to: Option<Loc>, inc_pc: bool) -> Self {
        let mut cycle = Self::read(CopyFrom::Loc(Loc::Pc), db_to);

        if inc_pc {
            cycle[1].push(SubInstr::new(Loc::Pc, CopyFrom::Loc(Loc::Pc), Modif::Inc));
        }

        cycle
    }

    // fn with_pc_inc(mut self) -> Self {
    //     self[1].push(SubInstr::new(Loc::Pc, CopyFrom::Loc(Loc::Pc), Modif::Inc));
    //     self
    // }
}

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
struct InstrCycles(Vec<Cycle>);

impl InstrCycles {
    pub fn from_instr(instr: Instr) -> Self {
        match instr {
            Instr::Brk => Self(vec![
                Cycle::read_pc(Some(Loc::StemHold), true),
                Cycle::read_pc(None, true),
            ]),
            Instr::Rti => todo!(),
            Instr::Rts => todo!(),
            Instr::Jsr => todo!(),
            Instr::JmpInd => todo!(),
            Instr::JmpAbs => todo!(),
            Instr::BranchOnSet(flag) => todo!(),
            Instr::BranchOnClear(flag) => todo!(),
            Instr::StackPush(reg) => todo!(),
            Instr::StackPull(reg) => todo!(),
            Instr::Impl(impl_op) => todo!(),
            Instr::Imm(gen_read_op) => todo!(),
            Instr::Abs(gen_op) => todo!(),
            Instr::Zpg(gen_op) => todo!(),
            Instr::AbsIdx(reg, gen_op) => todo!(),
            Instr::ZpgIdx(reg, gen_op) => todo!(),
            Instr::XInd(gen_op) => todo!(),
            Instr::IndY(gen_op) => todo!(),
            Instr::Illegal => todo!(),
        }
    }
}
