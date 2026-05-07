mod control;
mod instructions;
mod lines;
mod ram;
mod registers;

use crate::{
    RiotConnectionIds, RiotLines,
    riot::{
        instructions::PossibleInstructions,
        lines::{RiotLineStates, RiotOutputLines},
        ram::Ram,
        registers::{RegisterChanges, Registers},
    },
};
use emutils::line::{LineError, LineState};

const TIMER_INTERVALS: [usize; 4] = [1, 8, 64, 1024];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Riot {
    con: RiotConnectionIds,
    reg: Registers,
    ram: Ram,
    phi2_signal: bool,
    old_pa7_state: LineState,
}

impl Riot {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(connections: RiotConnectionIds) -> Self {
        Self {
            con: connections,
            reg: Registers::new(),
            ram: Ram::new(),
            phi2_signal: false,
            old_pa7_state: LineState::new(false, false, true),
        }
    }

    pub fn drive_phi2(&mut self, lines: RiotLines, bool_signal: bool) -> Result<(), LineError> {
        lines.check_valid()?;
        let states = RiotLineStates::from(&lines);
        let mut lines = RiotOutputLines::from(lines);

        match (self.phi2_signal, bool_signal) {
            (false, true) => self.handle_rising_edge(&mut lines, &states)?,
            (true, false) => {
                for (line, con) in lines.db.iter_mut(self.con.db)? {
                    line.set_all(con, false, false, true).unwrap();
                }
            }
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
        lines.db.remove_all(self.con.db)?;
        lines.pa.remove_all(self.con.pa)?;
        lines.pb.remove_all(self.con.pb)?;

        self.update_peripherals(lines)?;
        self.update_edc(states);
        self.update_timer();

        let mut reg_changes = RegisterChanges::default();

        if states.cs1.could_read_low() || states.cs2.could_read_high() {
            reg_changes.add_new_option();
        }

        if states.cs1.could_read_high() && states.cs2.could_read_low() {
            let instructions = PossibleInstructions::from(states);

            macro_rules! call_instr_fns {
                ($(($instr:ident, $action:expr)),+ $(,)?) => {$(
                    if instructions.$instr {
                        $action;
                    }
                )+};
            }

            call_instr_fns!(
                (ram, self.call_ram(lines, states)?),
                (io, self.call_io(lines, states)?),
                (wt, self.write_timer(states)),
                (rt, self.read_timer(lines)?),
                (rirf, self.read_ir_flag(lines)?),
                (wedc, self.write_edc(states)),
            );
        }

        Ok(())
    }
}
