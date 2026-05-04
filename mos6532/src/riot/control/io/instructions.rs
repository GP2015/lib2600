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
            instr_branch!(
                a1,
                instr_branch!(rw, write_ora, read_ora),
                instr_branch!(rw, write_orb, read_orb),
            ),
            instr_branch!(
                a1,
                instr_branch!(rw, write_ddra, read_ddra),
                instr_branch!(rw, write_ddrb, read_ddrb),
            ),
        );

        instructions
    }
}
