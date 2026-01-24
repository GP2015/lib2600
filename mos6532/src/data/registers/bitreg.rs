use crate::data::registers::state::RegBitState;

pub struct BitReg {
    state: RegBitState,
}

impl BitReg {
    pub fn new() -> Self {
        Self {
            state: RegBitState::Undefined,
        }
    }

    pub fn get(&self) -> RegBitState {
        self.state
    }

    pub fn set(&mut self, state: RegBitState) {
        self.state = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn reg() -> BitReg {
        BitReg::new()
    }

    #[rstest]
    fn get_initial(reg: BitReg) {
        assert_eq!(reg.get(), RegBitState::Undefined);
    }

    #[rstest]
    #[case(RegBitState::True)]
    fn set_get(mut reg: BitReg, #[case] state: RegBitState) {
        reg.set(state);
        assert_eq!(reg.get(), state);
    }
}
