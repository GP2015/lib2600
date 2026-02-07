use delegate::delegate;

use crate::pin::{
    PinError, PinState, SinglePin, SinglePinOutput,
    single::{CallbackFn, SinglePinSetup, core::PinCore},
};

pub struct MockPin<E> {
    core: PinCore<E>,
}

impl<E: From<PinError>> SinglePinSetup<E> for MockPin<E> {
    fn new(name: String) -> Self {
        Self {
            core: PinCore::new(name, PinState::Undefined),
        }
    }

    delegate! {
        to self.core {
            fn assign_callback(&mut self, callback: Option<Box<dyn CallbackFn<E>>>);
        }
    }
}

impl<E: From<PinError>> SinglePin<E> for MockPin<E> {
    delegate! {
        to self.core {
            fn state(&self) -> PinState;
            fn state_as_bool(&self) -> Option<bool>;
            fn read(&self) -> Result<bool, E>;
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), E> {
        self.core.set_signal(state)
    }

    fn drive_in(&mut self, state: bool) -> Result<(), E> {
        self.core.set_signal(PinState::from_bool(state))
    }

    fn tri_state_in(&mut self) -> Result<(), E> {
        self.core.set_signal(PinState::TriState)
    }

    fn undefine_in(&mut self) -> Result<(), E> {
        self.core.set_signal(PinState::Undefined)
    }
}

impl<E: From<PinError>> SinglePinOutput<E> for MockPin<E> {
    fn signal_out(&mut self, state: PinState) -> Result<(), E> {
        self.core.set_signal(state)
    }

    fn drive_out(&mut self, state: bool) -> Result<(), E> {
        self.core.set_signal(PinState::from_bool(state))
    }

    fn tri_state_out(&mut self) -> Result<(), E> {
        self.core.set_signal(PinState::TriState)
    }

    fn undefine_out(&mut self) -> Result<(), E> {
        self.core.set_signal(PinState::Undefined)
    }
}
