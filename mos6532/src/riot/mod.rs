mod control;
mod instructions;
mod lines;
mod ram;
mod states;

use crate::{
    RiotConnectionIds, RiotLines,
    riot::{
        instructions::PossibleInstructions, lines::RiotOutputLines, ram::Ram,
        states::RiotLineStates,
    },
};
use emutils::{
    line::{LineError, LineState},
    reg::{BitRegister, MBitRegister},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Riot {
    ram: Ram,
    con: RiotConnectionIds,
    phi2_signal: bool,
    pa7_prev_cycle: LineState,

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
    #[allow(clippy::missing_panics_doc)]
    pub fn new(connections: RiotConnectionIds) -> Self {
        let mut riot = Self {
            ram: Ram::new(),
            con: connections,
            phi2_signal: false,

            pa7_prev_cycle: LineState::new(false, false, true),

            ddra: MBitRegister::new("DDRA"),
            ddrb: MBitRegister::new("DDRB"),
            ora: MBitRegister::new("ORA"),
            orb: MBitRegister::new("ORB"),
            edc_enables_irq: BitRegister::new("EDC Enables IRQ"),
            edc_edge_type: BitRegister::new("EDC Edge Type"),
            edc_interrupt_flag: BitRegister::new("EDC Interrupt Flag"),
            timer_interrupt_flag: BitRegister::new("Timer Interrupt Flag"),
        };

        riot.ddra.add(0, true).expect("must fit");
        riot.ddrb.add(0, true).expect("must fit");
        riot.ora.add(0, true).expect("must fit");
        riot.orb.add(0, true).expect("must fit");

        riot.edc_enables_irq.add(false, true);
        riot.edc_edge_type.add(false, true);

        riot
    }

    pub fn drive_phi2(&mut self, lines: RiotLines, bool_signal: bool) -> Result<(), LineError> {
        lines.check_possible()?;
        let states = RiotLineStates::from(&lines);
        let mut lines = RiotOutputLines::from(lines);

        match (self.phi2_signal, bool_signal) {
            (false, true) => self.handle_rising_edge(&mut lines, &states)?,
            (true, false) => lines.db.add_high_z(self.con.db, true),
            _ => return Ok(()),
        }

        self.phi2_signal = bool_signal;
        Ok(())
    }

    fn handle_rising_edge(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
    ) -> Result<(), LineError> {
        let (cs1_low, cs1_high) = states.cs1.could_read_low_high();
        let (cs2_low, cs2_high) = states.cs2.could_read_low_high();

        if cs1_high && cs2_low {
            let only_selected = !(cs1_low || cs2_high);
            self.call_chip(lines, states, only_selected)?;
        } else {
            lines.db.add_high_z(self.con.db, true);
        }

        self.update_peripherals(lines)
    }

    fn call_chip(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        only_selected: bool,
    ) -> Result<(), LineError> {
        if states.rw.could_read_high() {
            lines.db.clear_only_possible(self.con.db);
        } else {
            lines.db.add_high_z(self.con.db, true);
        }

        let instructions = PossibleInstructions::from(states);
        let only_instruction = only_selected & instructions.only_instruction();

        macro_rules! call_instr_fns {
            ($(($instr:ident, $action:expr)),+ $(,)?) => {$(
                if instructions.$instr {
                    $action;
                }
            )+};
        }

        call_instr_fns!(
            (ram, self.call_ram(lines, states, only_instruction)?),
            (io, self.call_io(lines, states, only_instruction)?),
            (wt, self.write_timer(lines, states, only_instruction)?),
            (rt, self.read_timer(lines, states, only_instruction)?),
            (rirf, self.read_ir_flag(lines, only_instruction)?),
            (wedc, self.write_edc(lines, states, only_instruction)?),
        );

        Ok(())
    }
}
