use delegate::delegate;

use crate::pin::{
    PinError, PinSignal, PinState, SinglePinCore, SinglePinInterface, SinglePinOutput,
    single::core::PinCore,
};

pub struct MockPin<E> {
    core: PinCore<E>,
}

impl<E: From<PinError>> SinglePinInterface<E> for MockPin<E> {
    delegate! {
        to self.core {
            fn state(&self) -> PinState;
            fn prev_state(&self) -> PinState;
        }
    }

    fn add_signal_in(&mut self, state: PinSignal) -> Result<(), E> {
        self.core.set_signal(state);
        Ok(())
    }

    fn add_drive_in(&mut self, state: bool) -> Result<(), E> {
        self.core.set_signal(PinSignal::from_bool(state));
        Ok(())
    }

    fn add_tri_state_in(&mut self) {
        self.core.set_signal(PinSignal::TriState);
    }
}

impl<E> SinglePinCore for MockPin<E> {
    fn new(name: String) -> Self {
        Self {
            core: PinCore::new(name, PinSignal::Undefined),
        }
    }

    delegate! {
        to self.core {
            fn post_tick_update(&mut self);
        }
    }
}

impl<E: From<PinError>> SinglePinOutput<E> for MockPin<E> {
    fn add_signal_out(&mut self, state: PinSignal) -> Result<(), E> {
        self.core.set_signal(state);
        Ok(())
    }

    fn add_drive_out(&mut self, state: bool) -> Result<(), E> {
        self.core.set_signal(PinSignal::from_bool(state));
        Ok(())
    }

    fn add_tri_state_out(&mut self) {
        self.core.set_signal(PinSignal::TriState);
    }
}
