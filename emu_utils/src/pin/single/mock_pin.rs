use delegate::delegate;

use crate::pin::{
    PinError, PinState, SinglePinCore, SinglePinInterface, SinglePinOutput, single::core::PinCore,
};

pub struct MockPin<E> {
    core: PinCore<E>,
}

impl<E: From<PinError>> SinglePinInterface<E> for MockPin<E> {
    delegate! {
        to self.core {
            fn state(&self) -> PinState;
            fn prev_state(&self) -> PinState;
            fn state_as_bool(&self) -> Option<bool>;
            fn prev_state_as_bool(&self) -> Option<bool>;
            fn read(&self) -> Result<bool, E>;
            fn read_prev(&self) -> Result<bool, E>;
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), E> {
        self.core.set_signal(state);
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), E> {
        self.core.set_signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_in(&mut self) {
        self.core.set_signal(PinState::TriState);
    }

    fn undefine_in(&mut self) -> Result<(), E> {
        self.core.set_signal(PinState::Undefined);
        Ok(())
    }
}

impl<E> SinglePinCore for MockPin<E> {
    fn new(name: String) -> Self {
        Self {
            core: PinCore::new(name, PinState::Undefined),
        }
    }

    delegate! {
        to self.core {
            fn post_tick_update(&mut self);
        }
    }
}

impl<E: From<PinError>> SinglePinOutput<E> for MockPin<E> {
    fn signal_out(&mut self, state: PinState) -> Result<(), E> {
        self.core.set_signal(state);
        Ok(())
    }

    fn drive_out(&mut self, state: bool) -> Result<(), E> {
        self.core.set_signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_out(&mut self) {
        self.core.set_signal(PinState::TriState);
    }

    fn undefine_out(&mut self) -> Result<(), E> {
        self.core.set_signal(PinState::Undefined);
        Ok(())
    }
}
