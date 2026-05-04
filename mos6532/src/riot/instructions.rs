use crate::riot::states::RiotLineStates;

#[derive(Clone, Debug, Default)]
pub struct PossibleInstructions {
    pub nop: bool,
    pub reset: bool,
    pub ram: bool,
    pub io: bool,
    pub write_timer: bool,
    pub read_timer: bool,
    pub read_ir_flag: bool,
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
            self.read_ir_flag,
            self.write_edc,
        ]
        .into_iter()
        .filter(|&b| b)
        .count()
            < 2
    }
}

impl From<&RiotLineStates> for PossibleInstructions {
    fn from(states: &RiotLineStates) -> Self {
        let mut instructions = Self::default();

        let res = states.res;
        let cs1 = states.cs1;
        let cs2 = states.cs2;
        let rs = states.rs;
        let rw = states.rw;
        let a0 = states.a.line_state(0).expect("already checked");
        let a2 = states.a.line_state(2).expect("already checked");
        let a4 = states.a.line_state(4).expect("already checked");

        if res.could_read_low() {
            instructions.reset = true;
        }

        if res.could_read_high() {
            if cs1.could_read_low() || cs2.could_read_high() {
                instructions.nop = true;
            }

            if cs1.could_read_high() && cs2.could_read_low() {
                if rs.could_read_low() {
                    instructions.ram = true;
                }

                if rs.could_read_high() {
                    if a2.could_read_low() {
                        instructions.io = true;
                    }

                    if a2.could_read_high() {
                        if rw.could_read_low() {
                            if a4.could_read_low() {
                                instructions.write_edc = true;
                            }

                            if a4.could_read_high() {
                                instructions.write_timer = true;
                            }
                        }

                        if rw.could_read_high() {
                            if a0.could_read_low() {
                                instructions.read_timer = true;
                            }

                            if a0.could_read_high() {
                                instructions.read_ir_flag = true;
                            }
                        }
                    }
                }
            }
        }

        instructions
    }
}
