mod instructions;

use crate::{
    Riot,
    riot::{
        control::io::instructions::PossibleIoInstructions, lines::RiotOutputLines,
        states::RiotLineStates,
    },
};
use emutils::line::LineError;
use itertools::izip;

#[derive(Clone, Copy)]
enum AB {
    A,
    B,
}

impl Riot {
    pub(crate) fn update_peripherals(&self, lines: &mut RiotOutputLines) -> Result<(), LineError> {
        macro_rules! reg_to_p {
            ($p:expr, $bus_con:expr, $ddr:expr, $or:expr) => {
                for ((p_line, line_con), ddr_bit, or_bit) in
                    izip!($p.iter_mut($bus_con), $ddr.iter(), $or.iter())
                {
                    let ddr_high_possible = ddr_bit.high_possible();
                    let ddr_low_possible = ddr_bit.low_possible();

                    if ddr_high_possible {
                        p_line.copy_from_reg(line_con, or_bit, !ddr_low_possible)?;
                    }

                    if ddr_low_possible {
                        p_line.add_high_z(line_con, !ddr_high_possible);
                    }
                }
            };
        }

        reg_to_p!(lines.pa, self.con.pa, self.ddra, self.ora);
        reg_to_p!(lines.pb, self.con.pb, self.ddrb, self.orb);
        Ok(())
    }

    pub(crate) fn call_io(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        only_instruction: bool,
    ) -> Result<(), LineError> {
        let io_instructions = PossibleIoInstructions::from(states);
        let only_io = only_instruction & io_instructions.only_instruction();

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
            (woa, self.write_or(states, AB::A, only_io)),
            (roa, self.read_ora(lines, states, only_io)?),
            (wob, self.write_or(states, AB::B, only_io)),
            (rob, self.read_orb(lines, states, only_io)?),
            (wda, self.write_ddr(states, AB::A, only_io)),
            (rda, self.read_ddr(lines, AB::A, only_io)?),
            (wdb, self.write_ddr(states, AB::B, only_io)),
            (rdb, self.read_ddr(lines, AB::B, only_io)?),
        );

        Ok(())
    }

    fn write_ddr(&mut self, states: &RiotLineStates, ab: AB, only_io: bool) {
        match ab {
            AB::A => &mut self.ddra,
            AB::B => &mut self.ddrb,
        }
        .copy_from_bus_state(&states.db, only_io);
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

    fn write_or(&mut self, states: &RiotLineStates, ab: AB, only_io: bool) {
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
            .iter_mut(self.con.db)
            .zip(self.ddrb.iter())
            .enumerate()
        {
            let reg_could_read_high = reg.high_possible();
            let reg_could_read_low = reg.low_possible();

            if reg_could_read_low {
                line.copy_from_line_state(
                    line_con,
                    &states.pb.line_state(bit).expect("already checked"),
                    only_io && !reg_could_read_high,
                )?;
            }

            if reg_could_read_high {
                line.copy_from_reg(
                    line_con,
                    self.orb.bit(bit).expect("must be valid"),
                    only_io && !reg_could_read_low,
                )?;
            }
        }

        Ok(())
    }
}
