mod instructions;
mod ram;

use crate::{
    ClkCycle, RiotError, RiotLineRefs,
    riot::{instructions::PossibleInstructions, ram::Ram},
};
use emutils::{
    line::{BusConnection, LineConnection},
    reg::{BitRegister, MBitRegister},
};

#[allow(dead_code)]
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
    pub(crate) edc_enables_irq: BitRegister,
    pub(crate) edc_edge_type: BitRegister,
    pub(crate) edc_interrupt_flag: BitRegister,
    pub(crate) timer_interrupt_flag: BitRegister,
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
            edc_enables_irq: BitRegister::new("EDC Enables IRQ"),
            edc_edge_type: BitRegister::new("EDC Edge Type"),
            edc_interrupt_flag: BitRegister::new("EDC Interrupt Flag"),
            timer_interrupt_flag: BitRegister::new("Timer Interrupt Flag"),
        }
    }

    #[must_use]
    pub fn clk_cycle(&self) -> ClkCycle {
        self.clk_cycle
    }

    pub fn tick(&mut self, lines: &mut RiotLineRefs) -> Result<(), RiotError> {
        lines.check_bus_sizes()?;

        self.clk_cycle = match self.clk_cycle {
            ClkCycle::ClkLow => {
                let instructions = PossibleInstructions::from(lines);
                self.execute_possible_instructions(lines, &instructions)?;
                ClkCycle::ClkHigh
            }
            ClkCycle::ClkHigh => {
                lines
                    .db
                    .iter_mut(&self.db_con)
                    .for_each(|(line, con)| line.add_high_z(con, true));
                ClkCycle::ClkLow
            }
        };

        Ok(())
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
