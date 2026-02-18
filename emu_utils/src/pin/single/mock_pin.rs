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
    fn name(&self) -> &str {
        self.name.as_str()
    }

    delegate! {
        to self.signals{
            #[call(iter_all_enabled)]
            fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn possible_signals(&self) -> Vec<PinSignal>;

            fn collapsed(&self) -> Option<PinSignal>;
        }

        to self.prev_signals{
            #[call(iter_all_enabled)]
            fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn prev_possible_signals(&self) -> Vec<PinSignal>;

            #[call(collapsed)]
            fn prev_collapsed(&self) -> Option<PinSignal>;
        }

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

    fn set_possible_in_to_prev(&mut self) -> Result<(), E> {
        self.signals = self.prev_signals;
        Ok(())
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

    fn set_possible_out_to_prev(&mut self) -> Result<(), E> {
        self.signals = self.prev_signals;
        Ok(())
    }
}
