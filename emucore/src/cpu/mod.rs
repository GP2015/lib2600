pub mod reads;
pub mod regs;

use emucore_macros::cpu_instr_mnemonic;

use crate::{
    common::{
        HasCouldBe, HasMux,
        line::{
            multi::{BusDriveState, IsBusDriveState},
            single::DriveState,
        },
        read::multi::IsMultiRead,
        signal::LineSignal,
    },
    cpu::{
        reads::{CpuAllReads, CpuLineReads},
        regs::CpuRegs,
    },
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Cpu {
    pub phi2_out: DriveState,
    pub a_out: BusDriveState<13>,
    pub db_out: BusDriveState<8>,
    pub rw_out: DriveState,
    reg: CpuRegs,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            phi2_out: DriveState::none_enabled(),
            a_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            db_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            rw_out: LineSignal::HighZ.into(),
            reg: CpuRegs::new(),
        }
    }

    pub fn handle_rising_edge(&mut self, line_reads: CpuLineReads) {
        let reads = CpuAllReads::new(line_reads, self.reg.clone());
        todo!()
    }

    fn update_s(&mut self, reads: &CpuAllReads) {
        let def = &|| reads.reg.s;
        let set_to_x = &|| reads.reg.x;
        let dec = &|| reads.reg.s.decremented();
        let inc = &|| reads.reg.s.incremented();

        macro_rules! could_match {
            ($cond:expr, $def:expr, ($opt:expr, $branch:expr) $(,)?) => {
                HasMux::mux($cond.could_be($opt), $def, $branch)
            };
            (
                $cond:expr,
                $def:expr,
                ($opt1:expr, $branch1:expr),
                $(($other_opts:expr, $other_branches:expr)),+
                $(,)?
            ) => {
                HasMux::mux(
                    $cond.could_be($opt1),
                    &|| could_match!($cond, $def, $(($other_opts, $other_branches)),+),
                    $branch1
                )
            };
        }

        self.reg.s = could_match!(
            reads.reg.instr_cycle,
            def,
            (1, &|| could_match!(
                reads.line.db,
                def,
                (cpu_instr_mnemonic!(Txs), set_to_x),
            )),
            (2, &|| could_match!(
                reads.line.db,
                def,
                (cpu_instr_mnemonic!(Pha, Php, Brk), dec),
                (cpu_instr_mnemonic!(Pla, Plp, Rti, Rts), inc),
            )),
            (3, &|| could_match!(
                reads.line.db,
                def,
                (cpu_instr_mnemonic!(Brk, Jsr), dec),
                (cpu_instr_mnemonic!(Rti, Rts), inc),
            )),
            (4, &|| could_match!(
                reads.line.db,
                def,
                (cpu_instr_mnemonic!(Brk, Jsr), dec),
                (cpu_instr_mnemonic!(Rti), inc),
            )),
        );
    }

    pub fn handle_falling_edge(&mut self, line_reads: CpuLineReads) {
        let reads = CpuAllReads::new(line_reads, self.reg.clone());
        self.update_s(&reads);
        todo!()
    }
}
