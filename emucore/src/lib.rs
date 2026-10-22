#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod common;
mod cpu;
mod data;
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
    data::ExtDrives,
};

use crate::{
    cpu::Cpu,
    riot::{Riot, data::RiotLineReads},
};

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

    pub fn tick_rising(&mut self, drv: &ExtDrives) -> Result<(), LineError> {
        let rdy_read = BitState::Unknown;
        let cpu_out = self.cpu.tick_rising(rdy_read);

        todo!()
    }

    pub fn tick_falling(&mut self, drv: &ExtDrives) -> Result<(), LineError> {
        let riot_reads = self.riot_reads(drv)?;
        let riot_out = self.riot.tick(&riot_reads);

        // self.cpu.tick_falling(self.line_states.cpu_reads());

        todo!()
    }

    fn riot_reads(&self, drv: &ExtDrives) -> Result<RiotLineReads, LineError> {
        let mut pa: BitStringState<_> = [BitState::Unknown; _].into();

        let pa_low = drv
            .inp2
            .section::<4>(0)
            .read()
            .map_err(|bit| LineError::ImpossibleLineSignal(LineIdent::Inp2(bit)))?;

        let pa_high = drv
            .inp1
            .section::<4>(0)
            .read()
            .map_err(|bit| LineError::ImpossibleLineSignal(LineIdent::Inp1(bit)))?;

        pa[0..4].clone_from_slice(pa_low.as_slice());
        pa[4..8].clone_from_slice(pa_high.as_slice());

        let pb_ok = [
            (drv.res, LineIdent::Res),
            (drv.sel, LineIdent::Sel),
            (drv.col, LineIdent::Col),
            (drv.ldiff, LineIdent::LDiff),
            (drv.rdiff, LineIdent::RDiff),
        ]
        .map(|(line, ident)| line.read().ok_or(LineError::ImpossibleLineSignal(ident)));

        Ok(RiotLineReads {
            a: todo!(),
            db: todo!(),
            pa,
            pb: [pb_ok[0]?, pb_ok[1]?, pb_ok[2]?, pb_ok[3]?, pb_ok[4]?],
            cs1: todo!(),
            cs2: todo!(),
            rs: todo!(),
            rw: todo!(),
        })
    }
}
