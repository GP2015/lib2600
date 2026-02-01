use crate::pin::{PinError, PinState, SinglePin, SinglePinOutput, single::SinglePinNew};

pub struct MockPin<E> {
    state: PinState,
    err_type: std::marker::PhantomData<E>,
}

impl<E> MockPin<E> {
    fn signal(&mut self, state: PinState) {
        self.state = state;
    }
}

impl<E> SinglePinNew for MockPin<E> {
    fn new(_: String) -> Self {
        Self {
            state: PinState::Undefined,
            err_type: std::marker::PhantomData,
        }
    }
}

impl<E: From<PinError>> SinglePin for MockPin<E> {
    type Error = E;

    fn state(&self) -> PinState {
        self.state
    }

    fn read(&self) -> Result<bool, E> {
        match self.state {
            PinState::High => Ok(true),
            PinState::Low => Ok(false),
            _ => panic!(),
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), E> {
        self.signal(state);
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), E> {
        self.signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_in(&mut self) {
        self.signal(PinState::TriState);
    }

    fn undefine_in(&mut self) -> Result<(), E> {
        self.signal(PinState::Undefined);
        Ok(())
    }
}

impl<E: From<PinError>> SinglePinOutput for MockPin<E> {
    type Error = E;

    fn signal_out(&mut self, state: PinState) -> Result<(), E> {
        self.signal(state);
        Ok(())
    }

    fn drive_out(&mut self, state: bool) -> Result<(), E> {
        self.signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_out(&mut self) {
        self.signal(PinState::TriState);
    }

    fn undefine_out(&mut self) -> Result<(), E> {
        self.signal(PinState::Undefined);
        Ok(())
    }
}
