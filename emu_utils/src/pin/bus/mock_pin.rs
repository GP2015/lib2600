use crate::pin::{PinError, PinState, SinglePin, SinglePinOutput, single::SinglePinNew};

pub struct MockPin<E> {
    state: Option<PinState>,
    err_type: std::marker::PhantomData<E>,
}

impl<E> MockPin<E> {
    fn set_signal(&mut self, state: PinState) {
        self.state = Some(state);
    }
}

impl<E> SinglePinNew for MockPin<E> {
    fn new(_: String) -> Self {
        Self {
            state: None,
            err_type: std::marker::PhantomData,
        }
    }
}

impl<E: From<PinError>> SinglePin for MockPin<E> {
    type Error = E;

    fn read(&self) -> Result<bool, E> {
        match self.state {
            Some(PinState::High) => Ok(true),
            Some(PinState::Low) => Ok(false),
            _ => panic!(),
        }
    }

    fn state(&self) -> Option<PinState> {
        self.state
    }

    fn set_signal_in(&mut self, state: PinState) -> Result<(), E> {
        self.set_signal(state);
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), E> {
        self.set_signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_in(&mut self) {
        self.set_signal(PinState::TriState);
    }
}

impl<E: From<PinError>> SinglePinOutput for MockPin<E> {
    type Error = E;

    fn set_signal_out(&mut self, state: PinState) -> Result<(), E> {
        self.set_signal(state);
        Ok(())
    }

    fn drive_out(&mut self, state: bool) -> Result<(), E> {
        self.set_signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_out(&mut self) {
        self.set_signal(PinState::TriState);
    }
}
