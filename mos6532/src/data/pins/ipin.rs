use crate::data::pins::common::PinState;

pub struct InputPin {
    state: PinState,
}

impl InputPin {
    pub fn new() -> Self {
        Self {
            state: PinState::Undefined,
        }
    }

    pub fn read(&self) -> PinState {
        self.state
    }

    pub fn drive_in(&mut self, state: PinState) {
        self.state = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn reg() -> InputPin {
        InputPin::new()
    }

    #[rstest]
    fn read_initial(reg: InputPin) {
        assert_eq!(reg.read(), PinState::Undefined);
    }

    #[rstest]
    fn drive_read(mut reg: InputPin) {
        reg.drive_in(PinState::High);
        assert_eq!(reg.read(), PinState::High);
    }
}
