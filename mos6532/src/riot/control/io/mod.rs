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
    pub(crate) fn call_io(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        mut only_possible: bool,
    ) -> Result<(), LineError> {
        let instructions = PossibleIoInstructions::from(states);
        only_possible &= instructions.only_possible();
        self.execute_possible_io_instructions(lines, states, &instructions, only_possible)?;

        if instructions.write_ora | instructions.write_ddra {
            self.update_peripheral(lines, AB::A, only_possible)?;
        }

        if instructions.write_orb | instructions.write_ddrb {
            self.update_peripheral(lines, AB::B, only_possible)?;
        }

        Ok(())
    }

    fn execute_possible_io_instructions(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        instructions: &PossibleIoInstructions,
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
            (write_ora, self.write_or(states, AB::A, only_possible)),
            (read_ora, self.read_ora(lines, states, only_possible)?),
            (write_orb, self.write_or(states, AB::B, only_possible)),
            (read_orb, self.read_orb(lines, states, only_possible)?),
            (write_ddra, self.write_ddr(states, AB::A, only_possible)),
            (read_ddra, self.read_ddr(lines, AB::A, only_possible)?),
            (write_ddrb, self.write_ddr(states, AB::B, only_possible)),
            (read_ddrb, self.read_ddr(lines, AB::B, only_possible)?),
        );

        Ok(())
    }

    fn write_ddr(&mut self, states: &RiotLineStates, ab: AB, only_possible: bool) {
        match ab {
            AB::A => &mut self.ddra,
            AB::B => &mut self.ddrb,
        }
        .copy_from_bus_state(&states.db, only_possible)
        .expect("already checked");
    }

    fn read_ddr(
        &self,
        lines: &mut RiotOutputLines,
        ab: AB,
        only_possible: bool,
    ) -> Result<(), LineError> {
        lines.db.copy_from_reg(
            self.con.db,
            match ab {
                AB::A => &self.ddra,
                AB::B => &self.ddrb,
            },
            only_possible,
        )
    }

    fn write_or(&mut self, states: &RiotLineStates, ab: AB, only_possible: bool) {
        match ab {
            AB::A => &mut self.ora,
            AB::B => &mut self.orb,
        }
        .copy_from_bus_state(&states.db, only_possible)
        .expect("already checked");
    }

    fn read_ora(
        &self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        only_possible: bool,
    ) -> Result<(), LineError> {
        lines
            .db
            .copy_from_bus_state(self.con.db, &states.pa, only_possible)
    }

    fn read_orb(
        &self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        only_possible: bool,
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
                    only_possible && !reg_could_read_high,
                )?;
            }

            if reg_could_read_high {
                line.copy_from_reg(
                    line_con,
                    self.orb.bit(bit).expect("must be valid"),
                    only_possible && !reg_could_read_low,
                )?;
            }
        }

        Ok(())
    }

    fn update_peripheral(
        &self,
        lines: &mut RiotOutputLines,
        ab: AB,
        only_possible: bool,
    ) -> Result<(), LineError> {
        macro_rules! code_dupe {
            ($p:expr, $bus_con:expr, $ddr:expr, $or:expr) => {
                for ((p_line, line_con), ddr_bit, or_bit) in
                    izip!($p.iter_mut($bus_con), $ddr.iter(), $or.iter())
                {
                    if ddr_bit.high_possible() {
                        p_line.copy_from_reg(
                            line_con,
                            or_bit,
                            only_possible && !ddr_bit.low_possible(),
                        )?;
                    }
                }
            };
        }

        match ab {
            AB::A => code_dupe!(lines.pa, self.con.pa, self.ddra, self.ora),
            AB::B => code_dupe!(lines.pb, self.con.pb, self.ddrb, self.orb),
        }

        Ok(())
    }
}
