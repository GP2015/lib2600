mod instructions;
mod ram;

use crate::{
    RiotError,
    refs::RiotLineRefs,
    riot::{instructions::PossibleInstructions, ram::Ram},
};
use emutils::{
    line::{BusConnection, LineConnection},
    reg::MBitRegister,
};

#[derive(Default)]
enum ClkCycle {
    #[default]
    ClkLow,
    ClkHigh,
}

impl ClkCycle {
    pub fn step(&mut self) {
        *self = match self {
            Self::ClkLow => Self::ClkHigh,
            Self::ClkHigh => Self::ClkLow,
        }
    }
}

pub struct Riot {
    clk_cycle: ClkCycle,
    pub(crate) ram: Ram,
    pub(crate) db_con: BusConnection,
    pub(crate) pa_con: BusConnection,
    pub(crate) pb_con: BusConnection,
    pub(crate) irq_con: LineConnection,
    pub(crate) ddra: MBitRegister,
    pub(crate) ddrb: MBitRegister,
    pub(crate) ora: MBitRegister,
    pub(crate) orb: MBitRegister,
}

impl Riot {
    #[must_use]
    pub fn new(
        db_con: BusConnection,
        pa_con: BusConnection,
        pb_con: BusConnection,
        irq_con: LineConnection,
    ) -> Self {
        Self {
            clk_cycle: ClkCycle::default(),
            ram: Ram::new(),
            db_con,
            pa_con,
            pb_con,
            irq_con,
            ddra: MBitRegister::new("DDRA", 8),
            ddrb: MBitRegister::new("DDRB", 8),
            ora: MBitRegister::new("ORA", 8),
            orb: MBitRegister::new("ORB", 8),
        }
    }

    pub fn tick(&mut self, lines: &mut RiotLineRefs) -> Result<(), RiotError> {
        self.clk_cycle.step();

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
