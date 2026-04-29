use crate::{RiotLineRefs, error::RiotError};

#[derive(Debug, Default)]
pub struct PossibleIoInstructions {
    pub write_ora: bool,
    pub read_ora: bool,
    pub write_orb: bool,
    pub read_orb: bool,
    pub write_ddra: bool,
    pub read_ddra: bool,
    pub write_ddrb: bool,
    pub read_ddrb: bool,
}

impl PossibleIoInstructions {
    pub fn only_possible(&self) -> bool {
        [
            self.write_ora,
            self.read_ora,
            self.write_orb,
            self.read_orb,
            self.write_ddra,
            self.read_ddra,
            self.write_ddrb,
            self.read_ddrb,
        ]
        .into_iter()
        .filter(|&b| b)
        .count()
            < 2
    }

    pub fn from(lines: &mut RiotLineRefs) -> Result<Self, RiotError> {
        let mut instructions = Self::default();

        if lines.a.pin(0)?.could_read_low() {
            if lines.a.pin(1)?.could_read_low() {
                if lines.rw.could_read_low() {
                    instructions.write_ora = true;
                }

                if lines.rw.could_read_high() {
                    instructions.read_ora = true;
                }
            }

            if lines.a.pin(1)?.could_read_high() {
                if lines.rw.could_read_low() {
                    instructions.write_orb = true;
                }

                if lines.rw.could_read_high() {
                    instructions.read_orb = true;
                }
            }
        }

        if lines.a.pin(0)?.could_read_high() {
            if lines.a.pin(1)?.could_read_low() {
                if lines.rw.could_read_low() {
                    instructions.write_ddra = true;
                }

                if lines.rw.could_read_high() {
                    instructions.read_ddra = true;
                }
            }

            if lines.a.pin(1)?.could_read_high() {
                if lines.rw.could_read_low() {
                    instructions.write_ddrb = true;
                }

                if lines.rw.could_read_high() {
                    instructions.read_ddrb = true;
                }
            }
        }

        Ok(instructions)
    }
}
