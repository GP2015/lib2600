mod edc;
mod io;
mod ram;
mod reset;
mod timer;

use crate::{RIOT, RIOTError, data::AOrB};

impl RIOT {
    pub(super) fn tick(&mut self) -> Result<(), RIOTError> {
        if !self.buf.res.read()? {
            self.reset()?;
            return Ok(());
        }

        if !self.buf.cs1.read()? || self.buf.cs2.read()? {
            return Ok(());
        }

        self.decode_execute_instruction()?;

        self.update_edc();

        Ok(())
    }

    pub fn decode_execute_instruction(&mut self) -> Result<(), RIOTError> {
        match self.buf.rs.read()? {
            false => match self.buf.rw.read()? {
                false => self.write_ram()?,
                true => self.read_ram()?,
            },
            true => match self.buf.a.read_bit(2)? {
                false => match self.buf.a.read_bit(0)? {
                    false => match self.buf.a.read_bit(1)? {
                        false => match self.buf.rw.read()? {
                            false => self.write_or(AOrB::A)?,
                            true => self.read_ora()?,
                        },
                        true => match self.buf.rw.read()? {
                            false => self.write_or(AOrB::B)?,
                            true => self.read_orb()?,
                        },
                    },
                    true => match self.buf.a.read_bit(1)? {
                        false => match self.buf.rw.read()? {
                            false => self.write_ddr(AOrB::A)?,
                            true => self.read_ddr(AOrB::A)?,
                        },
                        true => match self.buf.rw.read()? {
                            false => self.write_ddr(AOrB::B)?,
                            true => self.read_ddr(AOrB::B)?,
                        },
                    },
                },
                true => match self.buf.rw.read()? {
                    false => match self.buf.a.read_bit(4)? {
                        false => (), // Write edge detect control (uses A1 and A0)
                        true => match self.buf.a.read_bit(1)? {
                            false => match self.buf.a.read_bit(0)? {
                                false => (), // Write timer +1T (uses A3)
                                true => (),  // Write timer +8T (uses A3)
                            },
                            true => match self.buf.a.read_bit(0)? {
                                false => (), // Write timer +64T (uses A3)
                                true => (),  // Write timer +1024T (uses A3)
                            },
                        },
                    },
                    true => match self.buf.a.read_bit(0)? {
                        false => (), // Read timer (uses A3)
                        true => (),  // Read interrupt flags
                    },
                },
            },
        };

        Ok(())
    }
}
