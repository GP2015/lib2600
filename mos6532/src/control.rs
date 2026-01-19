mod edc;
mod io;
mod misc;
mod ram;
mod timer;

use crate::{Riot, RiotError};

#[derive(PartialEq, Debug)]
enum Instruction {
    WriteRam,
    ReadRam,
    WriteDdra,
    ReadDdra,
    WriteDdrb,
    ReadDdrb,
    WriteOra,
    ReadOra,
    WriteOrb,
    ReadOrb,
    WriteTimer1T {
        enable_irq: bool,
    },
    WriteTimer8T {
        enable_irq: bool,
    },
    WriteTimer64T {
        enable_irq: bool,
    },
    WriteTimer1024T {
        enable_irq: bool,
    },
    ReadTimer,
    ReadInterruptFlag,
    WriteEdc {
        enable_irq: bool,
        use_pos_edge: bool,
    },
}

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

    fn decode_instruction(&mut self) -> Result<Instruction, RiotError> {
        let instruction = if self.buf.rs.read()? {
            if self.buf.a.read_bit(2)? {
                if self.buf.rw.read()? {
                    if self.buf.a.read_bit(0)? {
                        Instruction::ReadInterruptFlag
                    } else {
                        Instruction::ReadTimer
                    }
                } else if self.buf.a.read_bit(4)? {
                    let enable_irq = self.buf.a.read_bit(3)?;

                    if self.buf.a.read_bit(1)? {
                        if self.buf.a.read_bit(0)? {
                            Instruction::WriteTimer1024T { enable_irq }
                        } else {
                            Instruction::WriteTimer64T { enable_irq }
                        }
                    } else if self.buf.a.read_bit(0)? {
                        Instruction::WriteTimer8T { enable_irq }
                    } else {
                        Instruction::WriteTimer1T { enable_irq }
                    }
                } else {
                    let enable_irq = self.buf.a.read_bit(1)?;
                    let use_pos_edge = self.buf.a.read_bit(0)?;
                    Instruction::WriteEdc {
                        enable_irq,
                        use_pos_edge,
                    }
                }
            } else if self.buf.a.read_bit(0)? {
                if self.buf.a.read_bit(1)? {
                    if self.buf.rw.read()? {
                        Instruction::ReadDdrb
                    } else {
                        Instruction::WriteDdrb
                    }
                } else if self.buf.rw.read()? {
                    Instruction::ReadDdra
                } else {
                    Instruction::WriteDdra
                }
            } else if self.buf.a.read_bit(1)? {
                if self.buf.rw.read()? {
                    Instruction::ReadOrb
                } else {
                    Instruction::WriteOrb
                }
            } else if self.buf.rw.read()? {
                Instruction::ReadOra
            } else {
                Instruction::WriteOra
            }
        } else if self.buf.rw.read()? {
            Instruction::ReadRam
        } else {
            Instruction::WriteRam
        };

        Ok(instruction)
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), RiotError> {
        match instruction {
            Instruction::WriteRam => self.write_ram()?,
            Instruction::ReadRam => self.read_ram()?,
            Instruction::WriteDdra => self.write_ddra()?,
            Instruction::ReadDdra => self.read_ddra()?,
            Instruction::WriteDdrb => self.write_ddrb()?,
            Instruction::ReadDdrb => self.read_ddrb()?,
            Instruction::WriteOra => self.write_ora()?,
            Instruction::ReadOra => self.read_ora()?,
            Instruction::WriteOrb => self.write_orb()?,
            Instruction::ReadOrb => self.read_orb()?,
            Instruction::WriteTimer1T { enable_irq } => self.write_timer_1t(enable_irq)?,
            Instruction::WriteTimer8T { enable_irq } => self.write_timer_8t(enable_irq)?,
            Instruction::WriteTimer64T { enable_irq } => self.write_timer_64t(enable_irq)?,
            Instruction::WriteTimer1024T { enable_irq } => self.write_timer_1024t(enable_irq)?,
            Instruction::ReadTimer => self.read_timer()?,
            Instruction::ReadInterruptFlag => self.read_interrupt_flag()?,
            Instruction::WriteEdc {
                enable_irq,
                use_pos_edge,
            } => self.write_edc(enable_irq, use_pos_edge),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_write_ram() {
        let mut riot = Riot::new();
        riot.write_rs(false);
        riot.write_rw(false);
        assert_eq!(riot.decode_instruction().unwrap(), Instruction::WriteRam);
    }

    #[test]
    fn address_read_ram() {
        let mut riot = Riot::new();
        riot.write_rs(false);
        riot.write_rw(true);
        assert_eq!(riot.decode_instruction().unwrap(), Instruction::ReadRam);
    }
}
