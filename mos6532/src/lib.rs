#![warn(clippy::pedantic)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::cast_possible_truncation,
    clippy::similar_names
)]

mod control;
mod error;
mod instructions;
mod ram;

pub use crate::error::RiotError;

use crate::{instructions::PossibleInstructions, ram::Ram};
use emutils::{
    line::{Bus, BusConnection, Line},
    reg::MBitRegister,
};

pub struct RiotLineRefs<'a> {
    pub a: &'a Bus,
    pub db: &'a mut Bus,
    pub pa: &'a mut Bus,
    pub pb: &'a mut Bus,
    pub res: &'a Line,
    pub cs1: &'a Line,
    pub cs2: &'a Line,
    pub rs: &'a Line,
    pub rw: &'a Line,
    pub irq: &'a mut Line,
}

pub struct Riot {
    ram: Ram,
    db_con: BusConnection,
    pa_con: BusConnection,
    pb_con: BusConnection,
    ddra: MBitRegister,
    ddrb: MBitRegister,
    ora: MBitRegister,
    orb: MBitRegister,
}

impl Riot {
    #[must_use]
    pub fn new(db_con: BusConnection, pa_con: BusConnection, pb_con: BusConnection) -> Self {
        Self {
            ram: Ram::new(),
            db_con,
            pa_con,
            pb_con,
            ddra: MBitRegister::new("DDRA", 8),
            ddrb: MBitRegister::new("DDRB", 8),
            ora: MBitRegister::new("ORA", 8),
            orb: MBitRegister::new("ORB", 8),
        }
    }

    pub fn tick(&mut self, lines: &mut RiotLineRefs) -> Result<(), RiotError> {
        let instructions = PossibleInstructions::from(lines)?;
        self.execute_possible_instructions(lines, &instructions)
    }

    fn execute_possible_instructions(
        &mut self,
        lines: &mut RiotLineRefs,
        instructions: &PossibleInstructions,
    ) -> Result<(), RiotError> {
        let only_possible = instructions.only_possible();

        if instructions.reset {
            self.handle_reset(lines, only_possible);
        }

        if instructions.ram {
            self.handle_ram(lines, only_possible)?;
        }

        if instructions.io {
            self.handle_io(lines, only_possible)?;
        }

        if instructions.write_timer {
            self.handle_write_timer(lines, only_possible)?;
        }

        if instructions.read_timer {
            self.handle_read_timer(lines, only_possible)?;
        }

        if instructions.read_interrupt_flag {
            self.handle_read_interrupt_flag(lines, only_possible)?;
        }

        if instructions.write_edc {
            self.handle_write_edc(lines, only_possible)?;
        }

        Ok(())
    }
}
