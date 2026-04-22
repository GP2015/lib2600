mod edc;
mod instructions;
mod interrupt;
mod io;
mod ram;
mod reset;
mod timer;

use crate::{Riot, RiotError, control::instructions::PossibleInstructions};
use emutils::pin::{BusInputUI, PinInputUI};

impl Riot {
    // pub fn tick(&mut self) -> Result<(), RiotError> {
    //     match (self.pin.res.prev_state(), self.pin.res.state()) {
    //         (_, PinSignal::TriState) => return Err(RiotError::non_standard("tristated RES pin")),
    //         (PinSignal::Low, PinSignal::High) => self.on_rising_phi2_edge()?,
    //         (PinSignal::High, PinSignal::Low) => self.on_falling_phi2_edge()?,
    //         _ => (),
    //     };

    //     match (self.pin.phi2.prev_state(), self.pin.phi2.state()) {
    //         (_, PinSignal::TriState) => return Err(RiotError::non_standard("tristated PHI2 pin")),
    //         (_, PinSignal::Undefined) => return Err(RiotError::non_standard("undefined PHI2 pin")),
    //         (PinSignal::Low, PinSignal::High) => self.on_rising_phi2_edge()?,
    //         (PinSignal::High, PinSignal::Low) => self.on_falling_phi2_edge()?,
    //         _ => (),
    //     };

    //     Ok(())
    // }

    // fn on_rising_phi2_edge(&mut self) -> Result<(), RiotError> {
    //     if !self.res().read()? {
    //         self.reset()?;
    //         return Ok(());
    //     }

    //     if !self.cs1().read()? || self.cs2().read()? {
    //         return Ok(());
    //     }

    //     let instruction = self.decode_instruction()?;
    //     self.execute_instruction(instruction)
    // }

    // fn on_falling_phi2_edge(&mut self) -> Result<(), RiotError> {
    //     Ok(())
    // }

    pub(crate) fn possible_instructions(&mut self) -> PossibleInstructions {
        let mut instructions = PossibleInstructions::new();

        if self.pin.cs1.could_read_low() || self.pin.cs2.could_read_high() {
            instructions.nop = true;
        }

        if self.pin.cs1.could_read_high() && self.pin.cs2.could_read_low() {
            if self.pin.rs.could_read_low() {
                instructions.ram = true;
            }

            if self.pin.rs.could_read_high() {
                if self.pin.a.pin(2).expect("valid pin").could_read_low() {
                    instructions.io = true;
                }

                if self.pin.a.pin(2).expect("valid pin").could_read_high() {
                    if self.pin.rw.could_read_low() {
                        if self.pin.a.pin(4).expect("valid pin").could_read_low() {
                            instructions.write_edc = true;
                        }

                        if self.pin.a.pin(4).expect("valid pin").could_read_high() {
                            instructions.write_timer = true;
                        }
                    }

                    if self.pin.rw.could_read_high() {
                        if self.pin.a.pin(0).expect("valid pin").could_read_low() {
                            instructions.read_timer = true;
                        }

                        if self.pin.a.pin(0).expect("valid pin").could_read_high() {
                            instructions.read_interrupt_flag = true;
                        }
                    }
                }
            }
        }

        instructions
    }

    pub(crate) fn execute_possible_instructions(
        &mut self,
        instructions: &PossibleInstructions,
    ) -> Result<(), RiotError> {
        let only_possible = instructions.only_possible();

        if instructions.reset {
            self.handle_reset(only_possible)?;
        }

        if instructions.ram {
            self.handle_ram(only_possible)?;
        }

        if instructions.io {
            self.handle_io(only_possible)?;
        }

        if instructions.write_timer {
            self.handle_write_timer(only_possible)?;
        }

        if instructions.read_timer {
            self.handle_read_timer(only_possible)?;
        }

        if instructions.read_interrupt_flag {
            self.handle_read_interrupt_flag(only_possible)?;
        }

        if instructions.write_edc {
            self.handle_write_edc(only_possible)?;
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::{fixture, rstest};

//     #[fixture]
//     fn riot() -> Riot {
//         Riot::new()
//     }

//     #[rstest]
//     #[case(false, Instruction::WriteRam)]
//     #[case(true, Instruction::ReadRam)]
//     fn address_ram(mut riot: Riot, #[case] rw: bool, #[case] res: Instruction) {
//         riot.rs_mut().add_drive_in(false).unwrap();
//         riot.rw_mut().add_drive_in(rw).unwrap();
//         assert_eq!(riot.decode_instruction().unwrap(), res);
//     }

//     #[rstest]
//     #[case(false, false, true, Instruction::WriteDdra)]
//     #[case(true, false, true, Instruction::ReadDdra)]
//     #[case(false, true, true, Instruction::WriteDdrb)]
//     #[case(true, true, true, Instruction::ReadDdrb)]
//     #[case(false, false, false, Instruction::WriteOra)]
//     #[case(true, false, false, Instruction::ReadOra)]
//     #[case(false, true, false, Instruction::WriteOrb)]
//     #[case(true, true, false, Instruction::ReadOrb)]
//     fn address_io(
//         mut riot: Riot,
//         #[case] rw: bool,
//         #[case] a1: bool,
//         #[case] a0: bool,
//         #[case] instr: Instruction,
//     ) {
//         riot.rs_mut().add_drive_in(true).unwrap();
//         riot.rw_mut().add_drive_in(rw).unwrap();
//         riot.a_mut()
//             .pin_mut(2)
//             .unwrap()
//             .add_drive_in(false)
//             .unwrap();
//         riot.a_mut().pin_mut(1).unwrap().add_drive_in(a1).unwrap();
//         riot.a_mut().pin_mut(0).unwrap().add_drive_in(a0).unwrap();
//         assert_eq!(riot.decode_instruction().unwrap(), instr);
//     }

//     #[rstest]
//     #[case(false, false, false, Instruction::WriteTimer1T { enable_irq: false })]
//     #[case(false, false, true, Instruction::WriteTimer1T { enable_irq: true })]
//     #[case(false, true, false, Instruction::WriteTimer8T { enable_irq: false })]
//     #[case(false, true, true, Instruction::WriteTimer8T { enable_irq: true })]
//     #[case(true, false, false, Instruction::WriteTimer64T { enable_irq: false })]
//     #[case(true, false, true, Instruction::WriteTimer64T { enable_irq: true })]
//     #[case(true, true, false, Instruction::WriteTimer1024T { enable_irq: false })]
//     #[case(true, true, true, Instruction::WriteTimer1024T { enable_irq: true })]
//     fn address_write_timer(
//         mut riot: Riot,
//         #[case] a1: bool,
//         #[case] a0: bool,
//         #[case] enable_irq: bool,
//         #[case] instr: Instruction,
//     ) {
//         riot.rs_mut().add_drive_in(true).unwrap();
//         riot.rw_mut().add_drive_in(false).unwrap();
//         riot.a_mut().pin_mut(4).unwrap().add_drive_in(true).unwrap();
//         riot.a_mut()
//             .pin_mut(3)
//             .unwrap()
//             .add_drive_in(enable_irq)
//             .unwrap();
//         riot.a_mut().pin_mut(2).unwrap().add_drive_in(true).unwrap();
//         riot.a_mut().pin_mut(1).unwrap().add_drive_in(a1).unwrap();
//         riot.a_mut().pin_mut(0).unwrap().add_drive_in(a0).unwrap();
//         assert_eq!(riot.decode_instruction().unwrap(), instr);
//     }

//     #[rstest]
//     fn address_read_timer(mut riot: Riot, #[values(false, true)] enable_irq: bool) {
//         riot.rs_mut().add_drive_in(true).unwrap();
//         riot.rw_mut().add_drive_in(true).unwrap();
//         riot.a_mut()
//             .pin_mut(3)
//             .unwrap()
//             .add_drive_in(enable_irq)
//             .unwrap();
//         riot.a_mut().pin_mut(2).unwrap().add_drive_in(true).unwrap();
//         riot.a_mut()
//             .pin_mut(0)
//             .unwrap()
//             .add_drive_in(false)
//             .unwrap();
//         let instruction = Instruction::ReadTimer { enable_irq };
//         assert_eq!(instruction, riot.decode_instruction().unwrap(),);
//     }

//     #[rstest]
//     fn address_read_interrupt_flag(mut riot: Riot) {
//         riot.rs_mut().add_drive_in(true).unwrap();
//         riot.rw_mut().add_drive_in(true).unwrap();
//         riot.a_mut().pin_mut(2).unwrap().add_drive_in(true).unwrap();
//         riot.a_mut().pin_mut(0).unwrap().add_drive_in(true).unwrap();
//         let instruction = Instruction::ReadInterruptFlag;
//         assert_eq!(instruction, riot.decode_instruction().unwrap());
//     }

//     #[rstest]
//     fn address_write_edc(
//         mut riot: Riot,
//         #[values(false, true)] enable_irq: bool,
//         #[values(false, true)] use_pos_edge: bool,
//     ) {
//         riot.rs_mut().add_drive_in(true).unwrap();
//         riot.rw_mut().add_drive_in(false).unwrap();
//         riot.a_mut()
//             .pin_mut(4)
//             .unwrap()
//             .add_drive_in(false)
//             .unwrap();
//         riot.a_mut().pin_mut(2).unwrap().add_drive_in(true).unwrap();
//         riot.a_mut()
//             .pin_mut(1)
//             .unwrap()
//             .add_drive_in(enable_irq)
//             .unwrap();
//         riot.a_mut()
//             .pin_mut(0)
//             .unwrap()
//             .add_drive_in(use_pos_edge)
//             .unwrap();

//         let instruction = Instruction::WriteEdc {
//             enable_irq,
//             use_pos_edge,
//         };

//         assert_eq!(instruction, riot.decode_instruction().unwrap());
//     }
// }
