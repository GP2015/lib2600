mod instructions;

use crate::{Riot, RiotError, RiotLineRefs, control::io::instructions::PossibleIoInstructions};
use itertools::izip;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AB {
    A,
    B,
}

impl Riot {
    pub(crate) fn handle_io(
        &mut self,
        lines: &mut RiotLineRefs,
        mut only_possible: bool,
    ) -> Result<(), RiotError> {
        let instructions = PossibleIoInstructions::from(lines);
        only_possible &= instructions.only_possible();
        self.execute_possible_io_instructions(lines, &instructions, only_possible)?;

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
        lines: &mut RiotLineRefs,
        instructions: &PossibleIoInstructions,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        if instructions.write_ora {
            self.write_or(lines, AB::A, only_possible);
        }

        if instructions.read_ora {
            self.read_ora(lines, only_possible)?;
        }

        if instructions.write_orb {
            self.write_or(lines, AB::B, only_possible);
        }

        if instructions.read_orb {
            self.read_orb(lines, only_possible)?;
        }

        if instructions.write_ddra {
            self.write_ddr(lines, AB::A, only_possible);
        }

        if instructions.read_ddra {
            self.read_ddr(lines, AB::A, only_possible)?;
        }

        if instructions.write_ddrb {
            self.write_ddr(lines, AB::B, only_possible);
        }

        if instructions.read_ddrb {
            self.read_ddr(lines, AB::B, only_possible)?;
        }

        Ok(())
    }

    fn write_ddr(&mut self, lines: &mut RiotLineRefs, ab: AB, only_possible: bool) {
        match ab {
            AB::A => &mut self.ddra,
            AB::B => &mut self.ddrb,
        }
        .copy_from_bus(lines.db, only_possible)
        .expect("already checked");
    }

    fn read_ddr(
        &mut self,
        lines: &mut RiotLineRefs,
        ab: AB,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        lines
            .db
            .copy_from_reg(
                &self.db_con,
                match ab {
                    AB::A => &self.ddra,
                    AB::B => &self.ddrb,
                },
                only_possible,
            )
            .map_err(Into::into)
    }

    fn write_or(&mut self, lines: &mut RiotLineRefs, ab: AB, only_possible: bool) {
        match ab {
            AB::A => &mut self.ora,
            AB::B => &mut self.orb,
        }
        .copy_from_bus(lines.db, only_possible)
        .expect("already checked");
    }

    fn read_ora(&mut self, lines: &mut RiotLineRefs, only_possible: bool) -> Result<(), RiotError> {
        lines
            .db
            .copy_from_bus(&self.db_con, lines.pa, only_possible)
            .map_err(Into::into)
    }

    fn read_orb(&mut self, lines: &mut RiotLineRefs, only_possible: bool) -> Result<(), RiotError> {
        for (bit, ((line, line_con), reg)) in lines
            .db
            .iter_mut(&self.db_con)
            .zip(self.ddrb.iter())
            .enumerate()
        {
            let reg_could_read_high = reg.high_possible();
            let reg_could_read_low = reg.low_possible();

            if reg_could_read_low {
                line.copy_from_line(
                    line_con,
                    lines.pb.line(bit).expect("already checked"),
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
        &mut self,
        lines: &mut RiotLineRefs,
        ab: AB,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        let (p, bus_con, ddr, or) = match ab {
            AB::A => (&mut lines.pa, &self.pa_con, &self.ddra, &self.ora),
            AB::B => (&mut lines.pb, &self.pb_con, &self.ddrb, &self.orb),
        };

        for ((p_line, line_con), ddr_bit, or_bit) in
            izip!(p.iter_mut(bus_con), ddr.iter(), or.iter())
        {
            if ddr_bit.high_possible() {
                p_line.copy_from_reg(line_con, or_bit, only_possible && !ddr_bit.low_possible())?;
            }
        }

        Ok(())
    }
}
