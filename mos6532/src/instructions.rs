use crate::{RiotLineRefs, error::RiotError};

#[derive(Debug, Default)]
pub struct PossibleInstructions {
    pub nop: bool,
    pub reset: bool,
    pub ram: bool,
    pub io: bool,
    pub write_timer: bool,
    pub read_timer: bool,
    pub read_interrupt_flag: bool,
    pub write_edc: bool,
}

impl PossibleInstructions {
    pub fn only_possible(&self) -> bool {
        [
            self.nop,
            self.reset,
            self.ram,
            self.io,
            self.write_timer,
            self.read_timer,
            self.read_interrupt_flag,
            self.write_edc,
        ]
        .into_iter()
        .filter(|&b| b)
        .count()
            < 2
    }

    pub fn from(lines: &mut RiotLineRefs) -> Result<Self, RiotError> {
        let mut instructions = Self::default();

        if lines.cs1.could_read_low() || lines.cs2.could_read_high() {
            instructions.nop = true;
        }

        if lines.cs1.could_read_high() && lines.cs2.could_read_low() {
            if lines.rs.could_read_low() {
                instructions.ram = true;
            }

            if lines.rs.could_read_high() {
                if lines.a.pin(2)?.could_read_low() {
                    instructions.io = true;
                }

                if lines.a.pin(2)?.could_read_high() {
                    if lines.rw.could_read_low() {
                        if lines.a.pin(4)?.could_read_low() {
                            instructions.write_edc = true;
                        }

                        if lines.a.pin(4)?.could_read_high() {
                            instructions.write_timer = true;
                        }
                    }

                    if lines.rw.could_read_high() {
                        if lines.a.pin(0)?.could_read_low() {
                            instructions.read_timer = true;
                        }

                        if lines.a.pin(0)?.could_read_high() {
                            instructions.read_interrupt_flag = true;
                        }
                    }
                }
            }
        }

        Ok(instructions)
    }
}
