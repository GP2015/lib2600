mod instructions;
mod ram;

use crate::{
    RiotError, RiotLineRefs,
    riot::{instructions::PossibleInstructions, ram::Ram},
};
use emutils::{
    line::{BusConnection, BusState, LineConnection, LineState},
    reg::{BitRegister, MBitRegister},
};

#[allow(dead_code)]
pub struct Riot {
    pub(crate) ram: Ram,

    pub(crate) db_con: BusConnection,
    pub(crate) pa_con: BusConnection,
    pub(crate) pb_con: BusConnection,
    pub(crate) irq_con: LineConnection,

    a_prev: BusState,
    db_prev: BusState,
    pa_prev: BusState,
    pb_prev: BusState,
    phi2_prev: LineState,
    res_prev: LineState,
    cs1_prev: LineState,
    cs2_prev: LineState,
    rs_prev: LineState,
    rw_prev: LineState,
    irq_prev: LineState,

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
            ram: Ram::new(),

            db_con,
            pa_con,
            pb_con,
            irq_con,

            a_prev: lines.a.state(),
            db_prev: lines.db.state(),
            pa_prev: lines.pa.state(),
            pb_prev: lines.pb.state(),
            phi2_prev: lines.phi2.state(),
            res_prev: lines.res.state(),
            cs1_prev: lines.cs1.state(),
            cs2_prev: lines.cs2.state(),
            rs_prev: lines.rs.state(),
            rw_prev: lines.rw.state(),
            irq_prev: lines.irq.state(),

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

    pub fn tick(&mut self, lines: &mut RiotLineRefs) -> Result<(), RiotError> {
        lines.check_bus_sizes()?;

        let phi2_prev_low = self.phi2_prev.could_read_low();
        let phi2_prev_high = self.phi2_prev.could_read_high();
        let phi2_low = lines.phi2.could_read_low();
        let phi2_high = lines.phi2.could_read_high();

        let phi2_keep_low = phi2_prev_low && phi2_low;
        let phi2_keep_high = phi2_prev_high && phi2_high;
        let phi2_rising_edge = phi2_prev_low && phi2_high;
        let phi2_falling_edge = phi2_prev_high && phi2_low;

        let only_possible = [
            phi2_keep_low,
            phi2_keep_high,
            phi2_rising_edge,
            phi2_falling_edge,
        ]
        .into_iter()
        .filter(|&b| b)
        .count()
            == 1;

        if phi2_rising_edge {
            let instructions = PossibleInstructions::from(lines);
            self.execute_possible_instructions(
                lines,
                &instructions,
                only_possible & instructions.only_possible(),
            )?;
        }

        if phi2_falling_edge {
            lines.db.add_high_z(&self.db_con, only_possible);
        }

        Ok(())
    }

    fn execute_possible_instructions(
        &mut self,
        lines: &mut RiotLineRefs,
        instructions: &PossibleInstructions,
        only_possible: bool,
    ) -> Result<(), RiotError> {
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
