use crate::data::registers::common::RegBitState;

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
    fn set_get(mut reg: BitReg) {
        reg.set(RegBitState::High);
        assert_eq!(reg.get(), RegBitState::High);
    }
}
