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
    prev_phi2_state: LineState,

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
    pub fn new(connections: RiotConnectionIds) -> Self {
        Self {
            ram: Ram::new(),
            con: connections,
            prev_phi2_state: LineState::new(false, false, true),

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

    pub fn tick(&mut self, lines: RiotLines) -> Result<(), LineError> {
        let states = RiotLineStates::from(&lines);
        let mut lines = RiotOutputLines::from(lines);

        let phi2_prev_low = self.prev_phi2_state.could_read_low();
        let phi2_prev_high = self.prev_phi2_state.could_read_high();
        let phi2_low = states.phi2.could_read_low();
        let phi2_high = states.phi2.could_read_high();

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

        let mut execute_instr = |only_possible: bool| {
            let instructions = PossibleInstructions::from(&states);
            self.execute_possible_instructions(
                &mut lines,
                &states,
                &instructions,
                only_possible & instructions.only_possible(),
            )
        };

        if phi2_rising_edge {
            execute_instr(only_possible)?;
        }

        if phi2_keep_high {
            execute_instr(false)?;
        }

        if phi2_falling_edge {
            lines.db.add_high_z(self.con.db, only_possible);
        }

        self.prev_phi2_state = states.phi2;

        Ok(())
    }

    fn execute_possible_instructions(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        instructions: &PossibleInstructions,
        only_possible: bool,
    ) -> Result<(), LineError> {
        macro_rules! call_instr_fns {
            ($(($instr:ident, $action:expr)),+ $(,)?) => {
                $(
                    if instructions.$instr{
                        $action;
                    }
                )+
            };
        }

        call_instr_fns!(
            (reset, self.call_reset(lines, only_possible)),
            (ram, self.call_ram(lines, states, only_possible)?),
            (io, self.call_io(lines, states, only_possible)?),
            (write_timer, self.write_timer(lines, states, only_possible)?),
            (read_timer, self.read_timer(lines, states, only_possible)?),
            (read_ir_flag, self.read_ir_flag(lines, only_possible)?),
            (write_edc, self.write_edc(lines, states, only_possible)?),
        );

        Ok(())
    }
}
