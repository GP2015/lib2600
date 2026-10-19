#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod common;
mod cpu;
mod ext_drives;
mod line_reads;
mod riot;

pub use crate::{
    common::{
        bit::{multi::BitStringState, single::BitState},
        combine::Combine,
        cond::BaseCondition,
        line::{
            error::{LineError, LineIdent},
            output::{multi::BusOutput, single::LineOutput},
            signal::LineSignal,
        },
    },
    ext_drives::ExtDrives,
};

use crate::{cpu::Cpu, riot::Riot};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Emulator {
    cpu: Cpu,
    riot: Riot,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            riot: Riot::new(),
        }
    }

    pub fn tick(&mut self, ext: &ExtDrives) -> Result<(), LineError> {
        let rdy_read = BitState::Unknown;
        let cpu_out = self.cpu.tick_rising(rdy_read);

        // let riot_out = self.riot.tick(&self.line_states.riot_reads());

        // self.cpu.tick_falling(self.line_states.cpu_reads());

        Ok(())
    }
}
