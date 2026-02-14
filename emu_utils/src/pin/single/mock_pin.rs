use crate::pin::{
    PinError, PinSignal, SinglePinCore, SinglePinInterface, SinglePinOutput,
    possible::PossibleSignals,
};
use delegate::delegate;
use std::marker::PhantomData;

pub struct MockPin<E> {
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
    err_type: PhantomData<E>,
}

impl<E> SinglePinCore for MockPin<E> {
    fn new(name: String) -> Self {
        Self {
            name,
            signals: PossibleSignals::from(false, false, true),
            prev_signals: PossibleSignals::from(false, false, false),
            err_type: PhantomData,
        }
    }

    fn post_tick_update(&mut self) {
        self.prev_signals = self.signals;
        self.signals.set_all(false);
    }
}

impl<E: From<PinError>> SinglePinInterface<E> for MockPin<E> {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn possible_signals(&self) -> Vec<PinSignal> {
        self.signals.all_enabled()
    }

    fn prev_possible_signals(&self) -> Vec<PinSignal> {
        self.prev_signals.all_enabled()
    }

    fn collapsed(&self) -> Option<PinSignal> {
        self.signals.collapsed()
    }

    fn prev_collapsed(&self) -> Option<PinSignal> {
        self.prev_signals.collapsed()
    }

    delegate! {
        #[expr($; Ok(()))]
        to self.signals{
            #[call(set_signal)]
            fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), E>;

            #[call(set_bool_signal)]
            fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), E>;

            #[call(set_all)]
            fn set_all_signals_in(&mut self, possible: bool) -> Result<(), E>;
        }
    }

    fn set_tri_state_in(&mut self, possible: bool) {
        self.signals.tri_state = possible;
    }
}

impl<E: From<PinError>> SinglePinOutput<E> for MockPin<E> {
    delegate! {
        #[expr($; Ok(()))]
        to self.signals{
            #[call(set_signal)]
            fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), E>;

            #[call(set_bool_signal)]
            fn set_drive_out(&mut self, bool_signal: bool, possible: bool) -> Result<(), E>;

            #[call(set_all)]
            fn set_all_signals_out(&mut self, possible: bool) -> Result<(), E>;
        }
    }

    fn set_tri_state_out(&mut self, possible: bool) {
        self.signals.tri_state = possible;
    }
}
