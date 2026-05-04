use crate::riot::states::RiotLineStates;

#[derive(Clone, Debug, Default)]
pub struct PossibleIoInstructions {
    pub woa: bool,
    pub roa: bool,
    pub wob: bool,
    pub rob: bool,
    pub wda: bool,
    pub rda: bool,
    pub wdb: bool,
    pub rdb: bool,
}

impl PossibleIoInstructions {
    pub fn only_instruction(&self) -> bool {
        [
            self.woa, self.roa, self.wob, self.rob, self.wda, self.rda, self.wdb, self.rdb,
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
        let a1 = states.a.line_state(1).expect("already checked");

        macro_rules! instr_branch {
            ($state:expr, $low:ident, $high:ident $(,)?) => {
                instr_branch!($state, instructions.$low = true, instructions.$high = true)
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

        instr_branch!(
            a0,
            instr_branch!(a1, instr_branch!(rw, woa, roa), instr_branch!(rw, wob, rob),),
            instr_branch!(a1, instr_branch!(rw, wda, rda), instr_branch!(rw, wdb, rdb),),
        );

        instructions
    }
}
