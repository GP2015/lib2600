mod edc;
mod io;
mod misc;
mod ram;
mod timer;

use crate::{Riot, RiotError};

type InstructionPtr = fn(&mut Riot) -> Result<(), RiotError>;

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

        let instruction = self.decode_instruction()?;
        self.execute_instruction(instruction)
    }

    fn decode_instruction(&mut self) -> Result<InstructionPtr, RiotError> {
        let instruction = if self.buf.rs.read()? {
            if self.buf.a.read_bit(2)? {
                if self.buf.rw.read()? {
                    if self.buf.a.read_bit(0)? {
                        Self::read_interrupt_flag
                    } else {
                        Self::read_timer
                    }
                } else if self.buf.a.read_bit(4)? {
                    if self.buf.a.read_bit(1)? {
                        if self.buf.a.read_bit(0)? {
                            Self::write_timer_1024t
                        } else {
                            Self::write_timer_64t
                        }
                    } else if self.buf.a.read_bit(0)? {
                        Self::write_timer_8t
                    } else {
                        Self::write_timer_1t
                    }
                } else {
                    Self::write_edc
                }
            } else if self.buf.a.read_bit(0)? {
                if self.buf.a.read_bit(1)? {
                    if self.buf.rw.read()? {
                        Self::read_ddrb
                    } else {
                        Self::write_ddrb
                    }
                } else if self.buf.rw.read()? {
                    Self::read_ddra
                } else {
                    Self::write_ddra
                }
            } else if self.buf.a.read_bit(1)? {
                if self.buf.rw.read()? {
                    Self::read_orb
                } else {
                    Self::write_orb
                }
            } else if self.buf.rw.read()? {
                Self::read_ora
            } else {
                Self::write_ora
            }
        } else if self.buf.rw.read()? {
            Self::read_ram
        } else {
            Self::write_ram
        };

        Ok(instruction)
    }

    fn execute_instruction(&mut self, instruction: InstructionPtr) -> Result<(), RiotError> {
        instruction(self)
    }
}
