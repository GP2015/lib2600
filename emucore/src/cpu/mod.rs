pub mod reads;
pub mod regs;

use crate::{
    common::{
        CheckIs, HasMux,
        line::{
            multi::{BusDriveState, IsBusDriveState},
            single::DriveState,
        },
        read::multi::{IsMultiRead, MultiRead},
        signal::LineSignal,
    },
    cpu::{
        reads::{CpuAllReads, CpuLineReads},
        regs::CpuRegs,
    },
    mux_matches,
};
use emucore_macros::str_pattern_from_mnemonic;

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
        macro_rules! cond {
            (($($ic:literal),+), ($($mnem:expr),+)) => {
                r.reg.instr_cycle.is_any([$($ic),+])
                    & r.line.db.is_any(&str_pattern_from_mnemonic!($($mnem),+))
            };
        }

        self.reg.s = mux_matches!(
            (cond!((1), (Txs)), &|| r.reg.x),
            (
                cond!((2), (Pha, Php, Brk)) | cond!((3, 4), (Brk, Jsr)),
                &|| r.reg.s.decremented()
            ),
            (
                cond!((2), (Pla, Plp, Rti, Rts)) | cond!((3), (Rti, Rts)) | cond!((4), (Rti)),
                &|| r.reg.s.incremented()
            ),
            &|| r.reg.s
        );
    }

    pub fn handle_falling_edge(&mut self, line_reads: CpuLineReads) {
        let r = CpuAllReads::new(line_reads, self.reg.clone());
        self.update_s(&r);
        todo!()
    }
}
