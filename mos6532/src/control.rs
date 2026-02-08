mod edc;
mod interrupt;
mod io;
mod ram;
mod reset;
mod timer;

use crate::{Riot, RiotError};
use emu_utils::pin::{Bus, PinState, SinglePin};

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
    pub(crate) fn callback_res(
        &mut self,
        prev_state: PinState,
        state: PinState,
    ) -> Result<(), RiotError> {
        match (prev_state, state) {
            (_, PinState::TriState) => Err(RiotError::non_standard("tristated PHI2 pin")),
            (_, PinState::Undefined) => Err(RiotError::non_standard("undefined PHI2 pin")),
            (PinState::Low, PinState::High) => self.on_rising_phi2_edge(),
            (PinState::High, PinState::Low) => self.on_falling_phi2_edge(),
            _ => Ok(()),
        }
    }

    pub(crate) fn callback_phi2(
        &mut self,
        prev_state: PinState,
        state: PinState,
    ) -> Result<(), RiotError> {
        match (prev_state, state) {
            (_, PinState::TriState) => Err(RiotError::non_standard("tristated PHI2 pin")),
            (_, PinState::Undefined) => Err(RiotError::non_standard("undefined PHI2 pin")),
            (PinState::Low, PinState::High) => self.on_rising_phi2_edge(),
            (PinState::High, PinState::Low) => self.on_falling_phi2_edge(),
            _ => Ok(()),
        }
    }

    fn on_rising_phi2_edge(&mut self) -> Result<(), RiotError> {
        if !self.res().read()? {
            self.reset()?;
            return Ok(());
        }

        if !self.cs1().read()? || self.cs2().read()? {
            return Ok(());
        }

        let instruction = self.decode_instruction()?;
        self.execute_instruction(instruction)
    }

    fn on_falling_phi2_edge(&mut self) -> Result<(), RiotError> {
        Ok(())
    }

    fn decode_instruction(&mut self) -> Result<Instruction, RiotError> {
        let instruction = if self.rs().read()? {
            if self.a().pin(2)?.read()? {
                if self.rw().read()? {
                    if self.a().pin(0)?.read()? {
                        Instruction::ReadInterruptFlag
                    } else {
                        let enable_irq = self.a().pin(3)?.read()?;
                        Instruction::ReadTimer { enable_irq }
                    }
                } else if self.a().pin(4)?.read()? {
                    let enable_irq = self.a().pin(3)?.read()?;

                    if self.a().pin(1)?.read()? {
                        if self.a().pin(0)?.read()? {
                            Instruction::WriteTimer1024T { enable_irq }
                        } else {
                            Instruction::WriteTimer64T { enable_irq }
                        }
                    } else if self.a().pin(0)?.read()? {
                        Instruction::WriteTimer8T { enable_irq }
                    } else {
                        Instruction::WriteTimer1T { enable_irq }
                    }
                } else {
                    let enable_irq = self.a().pin(1)?.read()?;
                    let use_pos_edge = self.a().pin(0)?.read()?;
                    Instruction::WriteEdc {
                        enable_irq,
                        use_pos_edge,
                    }
                }
            } else if self.a().pin(0)?.read()? {
                if self.a().pin(1)?.read()? {
                    if self.rw().read()? {
                        Instruction::ReadDdrb
                    } else {
                        Instruction::WriteDdrb
                    }
                } else if self.rw().read()? {
                    Instruction::ReadDdra
                } else {
                    Instruction::WriteDdra
                }
            } else if self.a().pin(1)?.read()? {
                if self.rw().read()? {
                    Instruction::ReadOrb
                } else {
                    Instruction::WriteOrb
                }
            } else if self.rw().read()? {
                Instruction::ReadOra
            } else {
                Instruction::WriteOra
            }
        } else if self.rw().read()? {
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
    #[case(false, Instruction::WriteRam)]
    #[case(true, Instruction::ReadRam)]
    fn address_ram(mut riot: Riot, #[case] rw: bool, #[case] res: Instruction) {
        riot.rs_mut().drive_in(false).unwrap();
        riot.rw_mut().drive_in(rw).unwrap();
        assert_eq!(riot.decode_instruction().unwrap(), res);
    }

    #[rstest]
    #[case(false, false, true, Instruction::WriteDdra)]
    #[case(true, false, true, Instruction::ReadDdra)]
    #[case(false, true, true, Instruction::WriteDdrb)]
    #[case(true, true, true, Instruction::ReadDdrb)]
    #[case(false, false, false, Instruction::WriteOra)]
    #[case(true, false, false, Instruction::ReadOra)]
    #[case(false, true, false, Instruction::WriteOrb)]
    #[case(true, true, false, Instruction::ReadOrb)]
    fn address_io(
        mut riot: Riot,
        #[case] rw: bool,
        #[case] a1: bool,
        #[case] a0: bool,
        #[case] instr: Instruction,
    ) {
        riot.rs_mut().drive_in(true).unwrap();
        riot.rw_mut().drive_in(rw).unwrap();
        riot.a_mut().pin_mut(2).unwrap().drive_in(false).unwrap();
        riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
        riot.a_mut().pin_mut(0).unwrap().drive_in(a0).unwrap();
        assert_eq!(riot.decode_instruction().unwrap(), instr);
    }

    #[rstest]
    #[case(false, false, false, Instruction::WriteTimer1T { enable_irq: false })]
    #[case(false, false, true, Instruction::WriteTimer1T { enable_irq: true })]
    #[case(false, true, false, Instruction::WriteTimer8T { enable_irq: false })]
    #[case(false, true, true, Instruction::WriteTimer8T { enable_irq: true })]
    #[case(true, false, false, Instruction::WriteTimer64T { enable_irq: false })]
    #[case(true, false, true, Instruction::WriteTimer64T { enable_irq: true })]
    #[case(true, true, false, Instruction::WriteTimer1024T { enable_irq: false })]
    #[case(true, true, true, Instruction::WriteTimer1024T { enable_irq: true })]
    fn address_write_timer(
        mut riot: Riot,
        #[case] a1: bool,
        #[case] a0: bool,
        #[case] enable_irq: bool,
        #[case] instr: Instruction,
    ) {
        riot.rs_mut().drive_in(true).unwrap();
        riot.rw_mut().drive_in(false).unwrap();
        riot.a_mut().pin_mut(4).unwrap().drive_in(true).unwrap();
        riot.a_mut()
            .pin_mut(3)
            .unwrap()
            .drive_in(enable_irq)
            .unwrap();
        riot.a_mut().pin_mut(2).unwrap().drive_in(true).unwrap();
        riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
        riot.a_mut().pin_mut(0).unwrap().drive_in(a0).unwrap();
        assert_eq!(riot.decode_instruction().unwrap(), instr);
    }

    #[rstest]
    fn address_read_timer(mut riot: Riot, #[values(false, true)] enable_irq: bool) {
        riot.rs_mut().drive_in(true).unwrap();
        riot.rw_mut().drive_in(true).unwrap();
        riot.a_mut()
            .pin_mut(3)
            .unwrap()
            .drive_in(enable_irq)
            .unwrap();
        riot.a_mut().pin_mut(2).unwrap().drive_in(true).unwrap();
        riot.a_mut().pin_mut(0).unwrap().drive_in(false).unwrap();
        let instruction = Instruction::ReadTimer { enable_irq };
        assert_eq!(instruction, riot.decode_instruction().unwrap(),);
    }

    #[rstest]
    fn address_read_interrupt_flag(mut riot: Riot) {
        riot.rs_mut().drive_in(true).unwrap();
        riot.rw_mut().drive_in(true).unwrap();
        riot.a_mut().pin_mut(2).unwrap().drive_in(true).unwrap();
        riot.a_mut().pin_mut(0).unwrap().drive_in(true).unwrap();
        let instruction = Instruction::ReadInterruptFlag;
        assert_eq!(instruction, riot.decode_instruction().unwrap());
    }

    #[rstest]
    fn address_write_edc(
        mut riot: Riot,
        #[values(false, true)] enable_irq: bool,
        #[values(false, true)] use_pos_edge: bool,
    ) {
        riot.rs_mut().drive_in(true).unwrap();
        riot.rw_mut().drive_in(false).unwrap();
        riot.a_mut().pin_mut(4).unwrap().drive_in(false).unwrap();
        riot.a_mut().pin_mut(2).unwrap().drive_in(true).unwrap();
        riot.a_mut()
            .pin_mut(1)
            .unwrap()
            .drive_in(enable_irq)
            .unwrap();
        riot.a_mut()
            .pin_mut(0)
            .unwrap()
            .drive_in(use_pos_edge)
            .unwrap();

        let instruction = Instruction::WriteEdc {
            enable_irq,
            use_pos_edge,
        };

        assert_eq!(instruction, riot.decode_instruction().unwrap());
    }
}
