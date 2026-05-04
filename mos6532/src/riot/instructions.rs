use crate::riot::states::RiotLineStates;

#[derive(Clone, Debug, Default)]
pub struct PossibleInstructions {
    pub ram: bool,
    pub io: bool,
    pub wt: bool,
    pub rt: bool,
    pub rirf: bool,
    pub wedc: bool,
}

impl PossibleInstructions {
    pub fn only_instruction(&self) -> bool {
        [self.ram, self.io, self.wt, self.rt, self.rirf, self.wedc]
            .into_iter()
            .filter(|&b| b)
            .count()
            == 1
    }
}

impl From<&RiotLineStates> for PossibleInstructions {
    fn from(states: &RiotLineStates) -> Self {
        let mut instructions = Self::default();

        let rs = states.rs;
        let rw = states.rw;
        let a0 = states.a.line_state(0).expect("already checked");
        let a2 = states.a.line_state(2).expect("already checked");
        let a4 = states.a.line_state(4).expect("already checked");

        macro_rules! instr_branch {
            ($state:expr, $low:ident, $high:ident $(,)?) => {
                instr_branch!($state, instructions.$low = true, instructions.$high = true)
            };
            ($state:expr, $low:ident, $high_branch:expr $(,)?) => {
                instr_branch!($state, instructions.$low = true, $high_branch)
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
            rs,
            ram,
            instr_branch!(
                a2,
                io,
                instr_branch!(rw, instr_branch!(a4, wedc, wt), instr_branch!(a0, rt, rirf))
            )
        );

        instructions
    }
}
