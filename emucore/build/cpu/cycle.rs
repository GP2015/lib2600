use crate::build::cpu::instr::Instr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Loc {
    Phi2,
    Addr,
    Db,
    Rw,
    InstrCycles,
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
enum Modif {
    Inc,
    StackAddr,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Effect {
    cycle: u8,
    loc: Loc,
    kind: Modif,
}

impl Effect {
    const fn new(cycle: u8, loc: Loc, kind: Modif) -> Self {
        Self { cycle, loc, kind }
    }
}

struct InstrCycles {
    inc_c1_inc: bool,
    extra_effects: Vec<Effect>,
}

impl InstrCycles {
    pub const fn new(inc_c1_inc: bool) -> Self {
        Self {
            inc_c1_inc,
            extra_effects: Vec::new(),
        }
    }
}

impl InstrCycles {
    pub fn from_instr(instr: Instr) -> Self {
        macro_rules! effects {
            ($(($cycle:literal, $loc:ident, $mod:ident)),+ $(,)?) => {
                vec![$(Effect::new($cycle, Loc::$loc, Modif::$mod)),+]
            };
        }

        match instr {
            Instr::Brk => todo!(),
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
