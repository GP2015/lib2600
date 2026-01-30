use crate::{
    data::pins::{
        single::{SinglePin, SinglePinNew, SinglePinOutput},
        state::PinState,
    },
    error::RiotError,
};

pub struct MockPin {
    state: Option<PinState>,
}

impl MockPin {
    fn set_signal(&mut self, state: PinState) {
        self.state = Some(state);
    }
}

impl SinglePinNew for MockPin {
    fn new(_: String) -> Self {
        Self { state: None }
    }
}

impl SinglePin for MockPin {
    fn read(&self) -> Result<bool, RiotError> {
        match self.state {
            Some(PinState::High) => Ok(true),
            Some(PinState::Low) => Ok(false),
            _ => panic!(),
        }
    }

    fn state(&self) -> Option<PinState> {
        self.state
    }

    fn set_signal_in(&mut self, state: PinState) -> Result<(), RiotError> {
        self.set_signal(state);
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), RiotError> {
        self.set_signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_in(&mut self) {
        self.set_signal(PinState::TriState);
    }
}

impl SinglePinOutput for MockPin {
    fn set_signal_out(&mut self, state: PinState) -> Result<(), RiotError> {
        self.set_signal(state);
        Ok(())
    }

    fn drive_out(&mut self, state: bool) -> Result<(), RiotError> {
        self.set_signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_out(&mut self) {
        self.set_signal(PinState::TriState);
    }
}
