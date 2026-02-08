use delegate::delegate;

use crate::pin::{
    PinError, PinState, SinglePinInput, SinglePinOutput,
    single::{CallbackFn, SinglePinSetup, core::PinCore},
};

pub struct MockPin<O> {
    core: PinCore<O>,
}

impl<O> SinglePinSetup<O> for MockPin<O> {
    fn new(name: String) -> Self {
        Self {
            core: PinCore::new(name, PinState::Undefined),
        }
    }

    delegate! {
        to self.core {
            fn assign_callback(&mut self, callback: Box<CallbackFn<O>>);
        }
    }
}

impl<O> SinglePinInput for MockPin<O> {
    delegate! {
        to self.core {
            fn state(&self) -> PinState;
            fn state_as_bool(&self) -> Option<bool>;
            fn read(&self) -> Result<bool, PinError>;
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), PinError> {
        self.core.set_signal(state)
    }

    fn drive_in(&mut self, state: bool) -> Result<(), PinError> {
        self.core.set_signal(PinState::from_bool(state))
    }

    fn tri_state_in(&mut self) -> Result<(), PinError> {
        self.core.set_signal(PinState::TriState)
    }

    fn undefine_in(&mut self) -> Result<(), PinError> {
        self.core.set_signal(PinState::Undefined)
    }
}

impl<O> SinglePinOutput for MockPin<O> {
    fn signal_out(&mut self, state: PinState) -> Result<(), PinError> {
        self.core.set_signal(state)
    }

    fn drive_out(&mut self, state: bool) -> Result<(), PinError> {
        self.core.set_signal(PinState::from_bool(state))
    }

    fn tri_state_out(&mut self) -> Result<(), PinError> {
        self.core.set_signal(PinState::TriState)
    }

    fn undefine_out(&mut self) -> Result<(), PinError> {
        self.core.set_signal(PinState::Undefined)
    }
}
