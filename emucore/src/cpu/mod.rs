pub mod reads;
pub mod regs;

use crate::{
    common::{
        combine::mux_matches,
        cond::{base::BaseCondition, check::CheckIs},
        line::{multi::BusDriveState, single::DriveState},
        read::single::SingleRead,
        signal::LineSignal,
    },
    cpu::{
        reads::{CpuAllReads, CpuLineReads},
        regs::CpuRegs,
    },
};
use emucore_macros::mnem_pat;

macro_rules! ic {
    ($r:ident, $($v:literal),+) => {
        ($(BaseCondition::from($r.reg.instr_cycle[$v]))|+)
    };
}

macro_rules! db {
    ($r:ident, $($v:ident),+) => {
        $r.line.db.is_any(mnem_pat!($($v),+).iter())
    };
}

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
        let r = CpuAllReads::new(line_reads, self.reg.clone());
        todo!()
    }

    fn update_s(&mut self, r: &CpuAllReads) {
        self.reg.s = mux_matches!(
            (ic!(r, 1) & db!(r, Txs), &|| r.reg.x.clone()),
            (
                (ic!(r, 2) & db!(r, Pha, Php, Brk)) | (ic!(r, 3, 4) & db!(r, Brk, Jsr)),
                &|| r.reg.s.decremented()
            ),
            (
                (ic!(r, 2) & db!(r, Pla, Plp, Rti, Rts))
                    | (ic!(r, 3) & db!(r, Rti, Rts))
                    | (ic!(r, 4) & db!(r, Rti)),
                &|| r.reg.s.incremented()
            ),
            &|| r.reg.s.clone()
        );
    }

    pub fn handle_falling_edge(&mut self, line_reads: CpuLineReads) {
        let r = CpuAllReads::new(line_reads, self.reg.clone());
        self.update_s(&r);
        todo!()
    }
}
