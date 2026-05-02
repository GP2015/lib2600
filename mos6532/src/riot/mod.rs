mod control;
mod instructions;
mod ram;

use crate::{
    RiotError, RiotLineRefs,
    riot::{instructions::PossibleInstructions, ram::Ram},
};
use emutils::{
    line::{BusConnectionId, BusState, LineConnectionId, LineState},
    reg::{BitRegister, MBitRegister},
};

const INITIAL_PREV_LINE_STATE: LineState = LineState::new(false, false, true);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Riot {
    ram: Ram,

    db_con: BusConnectionId,
    pa_con: BusConnectionId,
    pb_con: BusConnectionId,
    irq_con: LineConnectionId,

    a_prev: BusState<7>,
    db_prev: BusState<8>,
    pa_prev: BusState<8>,
    pb_prev: BusState<8>,
    phi2_prev: LineState,
    res_prev: LineState,
    cs1_prev: LineState,
    cs2_prev: LineState,
    rs_prev: LineState,
    rw_prev: LineState,
    irq_prev: LineState,

    ddra: MBitRegister<8>,
    ddrb: MBitRegister<8>,
    ora: MBitRegister<8>,
    orb: MBitRegister<8>,
    edc_enables_irq: BitRegister,
    edc_edge_type: BitRegister,
    edc_interrupt_flag: BitRegister,
    timer_interrupt_flag: BitRegister,
}

impl Riot {
    #[must_use]
    pub fn new(
        db_con: BusConnectionId,
        pa_con: BusConnectionId,
        pb_con: BusConnectionId,
        irq_con: LineConnectionId,
    ) -> Self {
        Self {
            ram: Ram::new(),

            db_con,
            pa_con,
            pb_con,
            irq_con,

            a_prev: BusState::new([INITIAL_PREV_LINE_STATE; 7]),
            db_prev: BusState::new([INITIAL_PREV_LINE_STATE; 8]),
            pa_prev: BusState::new([INITIAL_PREV_LINE_STATE; 8]),
            pb_prev: BusState::new([INITIAL_PREV_LINE_STATE; 8]),
            phi2_prev: INITIAL_PREV_LINE_STATE,
            res_prev: INITIAL_PREV_LINE_STATE,
            cs1_prev: INITIAL_PREV_LINE_STATE,
            cs2_prev: INITIAL_PREV_LINE_STATE,
            rs_prev: INITIAL_PREV_LINE_STATE,
            rw_prev: INITIAL_PREV_LINE_STATE,
            irq_prev: INITIAL_PREV_LINE_STATE,

            ddra: MBitRegister::new("DDRA"),
            ddrb: MBitRegister::new("DDRB"),
            ora: MBitRegister::new("ORA"),
            orb: MBitRegister::new("ORB"),
            edc_enables_irq: BitRegister::new("EDC Enables IRQ"),
            edc_edge_type: BitRegister::new("EDC Edge Type"),
            edc_interrupt_flag: BitRegister::new("EDC Interrupt Flag"),
            timer_interrupt_flag: BitRegister::new("Timer Interrupt Flag"),
        }
    }

    pub fn tick(&mut self, lines: &mut RiotLineRefs) -> Result<(), RiotError> {
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
            let instructions = PossibleInstructions::new(lines);
            self.execute_possible_instructions(
                lines,
                &instructions,
                only_possible & instructions.only_possible(),
            )?;
        }

        if phi2_falling_edge {
            lines.db.add_high_z(self.db_con, only_possible);
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
            self.call_reset(lines, only_possible);
        }

        if instructions.ram {
            self.call_ram(lines, only_possible)?;
        }

        if instructions.io {
            self.call_io(lines, only_possible)?;
        }

        if instructions.write_timer {
            self.write_timer(lines, only_possible)?;
        }

        if instructions.read_timer {
            self.read_timer(lines, only_possible)?;
        }

        if instructions.read_interrupt_flag {
            self.read_interrupt_flag(lines, only_possible)?;
        }

        if instructions.write_edc {
            self.write_edc(lines, only_possible)?;
        }

        Ok(())
    }
}
