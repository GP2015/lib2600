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

        macro_rules! check_logic {
            ($state:expr, $low:ident, $high:ident $(,)?) => {
                check_logic!($state, instructions.$low = true, instructions.$high = true)
            };
            ($state:expr, $low:ident, $high_branch:expr $(,)?) => {
                check_logic!($state, instructions.$low = true, $high_branch)
            };
            ($state:expr, $low_branch:expr, $high_branch:expr $(,)?) => {{
                if $state.could_read_low() {
                    $low_branch
                }
                if $state.could_read_high() {
                    $high_branch
                }
            }};
        }

        check_logic!(res, reset, {
            if cs1.could_read_low() || cs2.could_read_high() {
                instructions.nop = true;
            }

            if cs1.could_read_high() && cs2.could_read_low() {
                check_logic!(
                    rs,
                    ram,
                    check_logic!(
                        a2,
                        io,
                        check_logic!(
                            rw,
                            check_logic!(a4, write_edc, write_timer),
                            check_logic!(a0, read_timer, read_ir_flag)
                        )
                    )
                );
            }
        });

        instructions
    }
}
