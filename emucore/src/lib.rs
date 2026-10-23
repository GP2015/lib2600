#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod common;
mod cpu;
mod data;
mod riot;

pub use crate::{
    common::{
        bit::{multi::BitStringState, single::BitState},
        combine::Combine,
        cond::BaseCond,
        line::{
            error::{Error, LineIdent},
            output::{multi::BusOutput, single::LineOutput},
            signal::LineSignal,
        },
    },
    data::{InputDrives, OutputDrives},
};

use crate::{
    cpu::Cpu,
    riot::{Riot, data::RiotLineReads},
};

macro_rules! read_line {
    ($line:expr, $id:ident) => {
        $line.read().ok_or(Error::NoSignal(LineIdent::$id))
    };

    ($line:expr, $id:ident, $bit:literal) => {
        $line.read().ok_or(Error::NoSignal(LineIdent::$id($bit)))
    };

    ($l0:expr, $l1:expr, $id:ident) => {
        $l0.contend_with($l1)
            .ok_or(Error::Short(LineIdent::$id))
            .map(|d| read_line!(d, $id))
    };

    ($l0:expr, $l1:expr, $id:ident, $bit:literal) => {
        $l0.contend_with($l1)
            .ok_or(Error::Short(LineIdent::$id($bit)))
            .map(|d| read_line!(d, $id, $bit))
    };
}

macro_rules! read_bus {
    ($bus:expr, $id:ident) => {
        $bus.read()
            .map_err(|bit| Error::NoSignal(LineIdent::$id(bit)))
    };

    ($b0:expr, $b1:expr, $id:ident) => {
        $b0.contend_with($b1)
            .map_err(|bit| Error::Short(LineIdent::$id(bit)))
            .map(|d| read_bus!(d, $id))
    };
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Emulator {
    cpu: Cpu,
    riot: Riot,

    cpu_a_out: BusOutput<13>,
    cpu_db_out: BusOutput<8>,

    rw_read: BitState,
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

            cpu_a_out: [LineSignal::HighZ; _].into(),
            cpu_db_out: [LineSignal::HighZ; _].into(),

            rw_read: BitState::Unknown,
        }

        // Read initial PC and tick rising
    }

    pub fn tick(&mut self, input_drv: &InputDrives) -> Result<OutputDrives, Error> {
        // Use CPU drives too

        self.riot.ack_new_pa7_drv(input_drv.inp1[3])?;
        let riot_reads = self.riot_reads(input_drv)?;
        let riot_out = self.riot.tick(&riot_reads);
        self.riot.ack_new_pa7_drv(input_drv.inp1[3])?;

        // Tick TIA
        let rdy_read = BitState::Unknown;

        // Combine drives and read.
        let db_read = riot_out
            .db
            .read()
            .map_err(|bit| Error::NoSignal(LineIdent::Db(bit)))?;

        self.cpu.tick_falling(rdy_read, db_read);

        // Rising edge below...

        // Acquire rdy_read
        let rdy_read = BitState::Unknown;

        let cpu_out = self.cpu.tick_rising(rdy_read);

        self.cpu_a_out = cpu_out.a.clone();
        self.cpu_db_out = cpu_out.db.clone();

        self.rw_read = cpu_out.rw.read().ok_or(Error::NoSignal(LineIdent::Rw))?;

        let mut inp1: BusOutput<_> = [LineSignal::HighZ; _].into();
        inp1[0..4].clone_from_slice(riot_out.pa.section::<4>(4).as_slice());

        let mut inp2: BusOutput<_> = [LineSignal::HighZ; _].into();
        inp2[0..4].clone_from_slice(riot_out.pa.section::<4>(0).as_slice());

        Ok(OutputDrives {
            a: cpu_out.a,
            db: cpu_out.db,
            inp1,
            inp2,
            rdiff: riot_out.pb[4],
            ldiff: riot_out.pb[3],
            col: riot_out.pb[2],
            sel: riot_out.pb[1],
            res: riot_out.pb[0],
        })
    }

    fn riot_reads(&self, drv: &InputDrives) -> Result<RiotLineReads, Error> {
        let mut pa: BitStringState<_> = [BitState::Unknown; _].into();
        pa[0..4].clone_from_slice(read_bus!(drv.inp2.section::<4>(0), Inp2)?.as_slice());
        pa[4..8].clone_from_slice(read_bus!(drv.inp1.section::<4>(0), Inp1)?.as_slice());

        let pb = [
            read_line!(drv.res, Res)?,
            read_line!(drv.sel, Sel)?,
            read_line!(drv.col, Col)?,
            read_line!(drv.ldiff, LDiff)?,
            read_line!(drv.rdiff, RDiff)?,
        ];

        Ok(RiotLineReads {
            a: read_bus!(&drv.a.section(0), &self.cpu_a_out.section(0), A)??,
            db: read_bus!(&drv.db, &self.cpu_db_out, Db)??,
            pa,
            pb,
            cs1: read_line!(drv.a[7], self.cpu_a_out[7], A, 7)??,
            cs2: read_line!(drv.a[12], self.cpu_a_out[12], A, 12)??,
            rs: read_line!(drv.a[9], self.cpu_a_out[9], A, 9)??,
            rw: self.rw_read,
        })
    }
}
