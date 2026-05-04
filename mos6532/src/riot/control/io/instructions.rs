use crate::riot::states::RiotLineStates;

#[derive(Clone, Debug, Default)]
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
}

impl From<&RiotLineStates> for PossibleIoInstructions {
    fn from(states: &RiotLineStates) -> Self {
        let mut instructions = Self::default();

        let rw = states.rw;
        let a0 = states.a.line_state(0).expect("already checked");
        let a1 = states.a.line_state(0).expect("already checked");

        if a0.could_read_low() {
            if a1.could_read_low() {
                if rw.could_read_low() {
                    instructions.write_ora = true;
                }

                if rw.could_read_high() {
                    instructions.read_ora = true;
                }
            }

            if a1.could_read_high() {
                if rw.could_read_low() {
                    instructions.write_orb = true;
                }

                if rw.could_read_high() {
                    instructions.read_orb = true;
                }
            }
        }

        if a0.could_read_high() {
            if a1.could_read_low() {
                if rw.could_read_low() {
                    instructions.write_ddra = true;
                }

                if rw.could_read_high() {
                    instructions.read_ddra = true;
                }
            }

            if a1.could_read_high() {
                if rw.could_read_low() {
                    instructions.write_ddrb = true;
                }

                if rw.could_read_high() {
                    instructions.read_ddrb = true;
                }
            }
        }

        instructions
    }
}
