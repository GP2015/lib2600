mod instructions;

use crate::{
    Riot,
    riot::{
        control::io::instructions::PossibleIoInstructions,
        lines::{RiotLineStates, RiotOutputLines},
    },
};
use emutils::line::LineError;

#[derive(Clone, Copy)]
enum AB {
    A,
    B,
}

impl Riot {
    pub(crate) fn call_io(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
    ) -> Result<(), LineError> {
        let io_instructions = PossibleIoInstructions::from(states);

        macro_rules! call_instr_fns {
            ($(($instr:ident, $action:expr)),+ $(,)?) => {
                $(
                    if io_instructions.$instr{
                        $action;
                    }
                )+
            };
        }

        call_instr_fns!(
            (woa, self.write_or(states, AB::A)),
            (roa, self.read_ora(lines, states)?),
            (wob, self.write_or(states, AB::B)),
            (rob, self.read_orb(lines, states)?),
            (wda, self.write_ddr(states, AB::A)),
            (rda, self.read_ddr(lines, AB::A)?),
            (wdb, self.write_ddr(states, AB::B)),
            (rdb, self.read_ddr(lines, AB::B)?),
        );

        Ok(())
    }

    fn write_ddr(&mut self, states: &RiotLineStates, ab: AB) {
        match ab {
            AB::A => &mut self.ddra,
            AB::B => &mut self.ddrb,
        }
        .copy_from_bus_state(&states.db, false);
    }

    fn read_ddr(
        &self,
        lines: &mut RiotOutputLines,
        ab: AB,
        only_io: bool,
    ) -> Result<(), LineError> {
        lines.db.copy_from_reg(
            self.con.db,
            match ab {
                AB::A => &self.ddra,
                AB::B => &self.ddrb,
            },
            only_io,
        )
    }

    fn write_or(&mut self, states: &RiotLineStates, ab: AB) {
        match ab {
            AB::A => &mut self.ora,
            AB::B => &mut self.orb,
        }
        .copy_from_bus_state(&states.db, only_io);
    }

    fn read_ora(
        &self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        only_io: bool,
    ) -> Result<(), LineError> {
        lines
            .db
            .copy_from_bus_state(self.con.db, &states.pa, only_io)
    }

    fn read_orb(
        &self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        only_io: bool,
    ) -> Result<(), LineError> {
        for (bit, ((line, line_con), reg)) in lines
            .db
            .iter_mut(self.con.db)?
            .zip(self.ddrb.iter())
            .enumerate()
        {
            let (reg_low, reg_high) = reg.low_high_possible();

            if reg_low {
                #[allow(clippy::unwrap_used)]
                line.copy_from_line_state(
                    line_con,
                    &states.pb.try_line_state(bit).unwrap(),
                    only_io && !reg_high,
                )?;
            }

            if reg_high {
                #[allow(clippy::unwrap_used)]
                line.copy_from_reg(
                    line_con,
                    self.orb.try_bit(bit).unwrap(),
                    only_io && !reg_low,
                )?;
            }
        }

        Ok(())
    }
}
