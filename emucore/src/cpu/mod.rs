pub mod reads;
pub mod regs;

include!(concat!(env!("OUT_DIR"), "/cpu_gen.rs"));

use crate::{
    common::{
        line::{multi::BusDriveState, single::DriveState},
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

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn handle_falling_edge(&mut self, line_reads: CpuLineReads) {
        let r = CpuAllReads::new(line_reads, self.reg.clone());
        todo!()
    }
}
