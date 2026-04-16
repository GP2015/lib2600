mod ab;
mod instructions;

use emu_utils::pin::{BusCore, BusOutput, SinglePinCore, SinglePinOutput};
use itertools::izip;

use crate::{
    Riot, RiotError,
    riot::control::io::{ab::AB, instructions::PossibleIoInstructions},
};

impl Riot {
    pub(crate) fn handle_io(&mut self, mut only_possible: bool) -> Result<(), RiotError> {
        let instructions = self.possible_io_instructions();
        only_possible &= instructions.only_possible();
        self.execute_possible_io_instructions(&instructions, only_possible)?;

        if instructions.write_ora | instructions.write_ddra {
            self.update_peripheral(AB::A, only_possible)?;
        }

        if instructions.write_orb | instructions.write_ddrb {
            self.update_peripheral(AB::B, only_possible)?;
        }

        Ok(())
    }

    fn possible_io_instructions(&mut self) -> PossibleIoInstructions {
        let mut instructions = PossibleIoInstructions::new();

        if self.pin.a.pin(0).expect("valid pin").could_read_low() {
            if self.pin.a.pin(1).expect("valid pin").could_read_low() {
                if self.pin.rw.could_read_low() {
                    instructions.write_ora = true;
                }

                if self.pin.rw.could_read_high() {
                    instructions.read_ora = true;
                }
            }

            if self.pin.a.pin(1).expect("valid pin").could_read_high() {
                if self.pin.rw.could_read_low() {
                    instructions.write_orb = true;
                }

                if self.pin.rw.could_read_high() {
                    instructions.read_orb = true;
                }
            }
        }

        if self.pin.a.pin(0).expect("valid pin").could_read_high() {
            if self.pin.a.pin(1).expect("valid pin").could_read_low() {
                if self.pin.rw.could_read_low() {
                    instructions.write_ddra = true;
                }

                if self.pin.rw.could_read_high() {
                    instructions.read_ddra = true;
                }
            }

            if self.pin.a.pin(1).expect("valid pin").could_read_high() {
                if self.pin.rw.could_read_low() {
                    instructions.write_ddrb = true;
                }

                if self.pin.rw.could_read_high() {
                    instructions.read_ddrb = true;
                }
            }
        }

        instructions
    }

    fn execute_possible_io_instructions(
        &mut self,
        instructions: &PossibleIoInstructions,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        if instructions.write_ora {
            self.write_or(AB::A, only_possible);
        }

        if instructions.read_ora {
            self.read_ora(only_possible)?;
        }

        if instructions.write_orb {
            self.write_or(AB::B, only_possible);
        }

        if instructions.read_orb {
            self.read_orb(only_possible)?;
        }

        if instructions.write_ddra {
            self.write_ddr(AB::A, only_possible);
        }

        if instructions.read_ddra {
            self.read_ddr(AB::A, only_possible)?;
        }

        if instructions.write_ddrb {
            self.write_ddr(AB::B, only_possible);
        }

        if instructions.read_ddrb {
            self.read_ddr(AB::B, only_possible)?;
        }

        Ok(())
    }

    fn write_ddr(&mut self, ab: AB, only_possible: bool) {
        match ab {
            AB::A => &mut self.reg.ddra,
            AB::B => &mut self.reg.ddrb,
        }
        .input_from_bus(&self.pin.db, only_possible)
    }

    fn read_ddr(&mut self, ab: AB, only_possible: bool) -> Result<(), RiotError> {
        self.pin.db.output_from_reg(
            match ab {
                AB::A => &self.reg.ddra,
                AB::B => &self.reg.ddrb,
            },
            only_possible,
        )
    }

    fn write_or(&mut self, ab: AB, only_possible: bool) {
        match ab {
            AB::A => &mut self.reg.ora,
            AB::B => &mut self.reg.orb,
        }
        .input_from_bus(&self.pin.db, only_possible);
    }

    fn read_ora(&mut self, only_possible: bool) -> Result<(), RiotError> {
        self.pin.db.output_from_bus(&self.pin.pa, only_possible)
    }

    fn read_orb(&mut self, only_possible: bool) -> Result<(), RiotError> {
        for (bit, (pin, reg)) in self.pin.db.iter_mut().zip(self.reg.ddrb.iter()).enumerate() {
            let reg_could_read_high = reg.high_possible();
            let reg_could_read_low = reg.low_possible();

            if reg_could_read_low {
                pin.output_from_pin(
                    self.pin.pb.pin(bit).expect("valid pin"),
                    only_possible && !reg_could_read_high,
                )?;
            }

            if reg_could_read_high {
                pin.output_from_reg(
                    self.reg.orb.bit(bit).expect("valid bit"),
                    only_possible && !reg_could_read_low,
                )?;
            }
        }

        Ok(())
    }

    fn update_peripheral(&mut self, ab: AB, only_possible: bool) -> Result<(), RiotError> {
        let (p, ddr, or) = match ab {
            AB::A => (&mut self.pin.pa, &self.reg.ddra, &self.reg.ora),
            AB::B => (&mut self.pin.pb, &self.reg.ddrb, &self.reg.orb),
        };

        for (p_pin, ddr_bit, or_bit) in izip!(p.iter_mut(), ddr.iter(), or.iter()) {
            if ddr_bit.high_possible() {
                p_pin.output_from_reg(or_bit, only_possible && !ddr_bit.low_possible())?;
            }
        }

        Ok(())
    }
}
