mod edc;
mod io;
mod misc;
mod ram;
mod timer;

use crate::{Riot, RiotError};

impl Riot {
    pub(super) fn tick(&mut self) -> Result<(), RiotError> {
        if !self.buf.res.read()? {
            self.reset()?;
            return Ok(());
        }

        if !self.buf.cs1.read()? || self.buf.cs2.read()? {
            return Ok(());
        }

        self.update_edc()?;

        self.decode_execute_instruction()?;

        Ok(())
    }

    fn decode_execute_instruction(&mut self) -> Result<(), RiotError> {
        if self.buf.rs.read()? {
            if self.buf.a.read_bit(2)? {
                if self.buf.rw.read()? {
                    if self.buf.a.read_bit(0)? {
                        self.read_interrupt_flag()?;
                    } else {
                        // Read timer (uses A3)
                    }
                } else if self.buf.a.read_bit(4)? {
                    if self.buf.a.read_bit(1)? {
                        if self.buf.a.read_bit(0)? {
                            // Write timer +1024T (uses A3)
                        } else {
                            // Write timer +64T (uses A3)
                        }
                    } else if self.buf.a.read_bit(0)? {
                        // Write timer +8T (uses A3)
                    } else {
                        // Write timer +1T (uses A3)
                    }
                } else {
                    let enable_irq = self.buf.a.read_bit(1)?;
                    let use_pos_edge = self.buf.a.read_bit(0)?;
                    self.write_edc(enable_irq, use_pos_edge);
                }
            } else if self.buf.a.read_bit(0)? {
                if self.buf.a.read_bit(1)? {
                    if self.buf.rw.read()? {
                        self.read_ddrb()?
                    } else {
                        self.write_ddrb()?
                    }
                } else if self.buf.rw.read()? {
                    self.read_ddra()?
                } else {
                    self.write_ddra()?
                }
            } else if self.buf.a.read_bit(1)? {
                if self.buf.rw.read()? {
                    self.read_orb()?
                } else {
                    self.write_orb()?
                }
            } else if self.buf.rw.read()? {
                self.read_ora()?
            } else {
                self.write_ora()?
            }
        } else if self.buf.rw.read()? {
            self.read_ram()?
        } else {
            self.write_ram()?
        }

        Ok(())
    }
}
