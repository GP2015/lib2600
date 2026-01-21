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
    ReadTimer {
        enable_irq: bool,
    },
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

        self.update_peripherals()?;

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
                        let enable_irq = self.buf.a.read_bit(3)?;
                        Instruction::ReadTimer { enable_irq }
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
            Instruction::ReadTimer { enable_irq } => self.read_timer(enable_irq)?,
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
    use rstest::{fixture, rstest};

    #[fixture]
    fn riot() -> Riot {
        Riot::new()
    }

    #[rstest]
    fn address_ram(mut riot: Riot) {
        riot.write_rs(false);
        riot.write_rw(false);
        assert_eq!(riot.decode_instruction().unwrap(), Instruction::WriteRam);
        riot.write_rw(true);
        assert_eq!(riot.decode_instruction().unwrap(), Instruction::ReadRam);
    }

    fn address_io(instr: Instruction, rw: bool, a1: bool, a0: bool) {
        let mut riot = Riot::new();
        riot.write_rs(true);
        riot.write_rw(rw);
        riot.write_a_bit(2, false).unwrap();
        riot.write_a_bit(1, a1).unwrap();
        riot.write_a_bit(0, a0).unwrap();
        assert_eq!(riot.decode_instruction().unwrap(), instr);
    }

    #[test]
    fn address_ddrs() {
        address_io(Instruction::WriteDdra, false, false, true);
        address_io(Instruction::ReadDdra, true, false, true);
        address_io(Instruction::WriteDdrb, false, true, true);
        address_io(Instruction::ReadDdrb, true, true, true);
    }

    #[test]
    fn address_ors() {
        address_io(Instruction::WriteOra, false, false, false);
        address_io(Instruction::ReadOra, true, false, false);
        address_io(Instruction::WriteOrb, false, true, false);
        address_io(Instruction::ReadOrb, true, true, false);
    }

    fn address_write_timer(instr: Instruction, enable_irq: bool, a1: bool, a0: bool) {
        let mut riot = Riot::new();
        riot.write_rs(true);
        riot.write_rw(false);
        riot.write_a_bit(4, true).unwrap();
        riot.write_a_bit(3, enable_irq).unwrap();
        riot.write_a_bit(2, true).unwrap();
        riot.write_a_bit(1, a1).unwrap();
        riot.write_a_bit(0, a0).unwrap();
        assert_eq!(riot.decode_instruction().unwrap(), instr);
    }

    #[rstest]
    #[case(false)]
    #[case(true)]
    fn address_write_timers(#[case] enable_irq: bool) {
        let instruction = Instruction::WriteTimer1T { enable_irq };
        address_write_timer(instruction, enable_irq, false, false);
        let instruction = Instruction::WriteTimer8T { enable_irq };
        address_write_timer(instruction, enable_irq, false, true);
        let instruction = Instruction::WriteTimer64T { enable_irq };
        address_write_timer(instruction, enable_irq, true, false);
        let instruction = Instruction::WriteTimer1024T { enable_irq };
        address_write_timer(instruction, enable_irq, true, true);
    }

    #[rstest]
    #[case(false)]
    #[case(true)]
    fn address_read_timer(mut riot: Riot, #[case] enable_irq: bool) {
        riot.write_rs(true);
        riot.write_rw(true);
        riot.write_a_bit(3, enable_irq).unwrap();
        riot.write_a_bit(2, true).unwrap();
        riot.write_a_bit(0, false).unwrap();
        let instruction = Instruction::ReadTimer { enable_irq };
        assert_eq!(instruction, riot.decode_instruction().unwrap(),);
    }

    #[rstest]
    fn address_read_interrupt_flag(mut riot: Riot) {
        riot.write_rs(true);
        riot.write_rw(true);
        riot.write_a_bit(2, true).unwrap();
        riot.write_a_bit(0, true).unwrap();
        let instruction = Instruction::ReadInterruptFlag;
        assert_eq!(instruction, riot.decode_instruction().unwrap());
    }

    #[rstest]
    #[case(false, false)]
    #[case(false, true)]
    #[case(true, false)]
    #[case(true, true)]
    fn address_write_edc(mut riot: Riot, #[case] enable_irq: bool, #[case] use_pos_edge: bool) {
        riot.write_rs(true);
        riot.write_rw(false);
        riot.write_a_bit(4, false).unwrap();
        riot.write_a_bit(2, true).unwrap();
        riot.write_a_bit(1, enable_irq).unwrap();
        riot.write_a_bit(0, use_pos_edge).unwrap();

        let instruction = Instruction::WriteEdc {
            enable_irq,
            use_pos_edge,
        };

        assert_eq!(instruction, riot.decode_instruction().unwrap());
    }
}
