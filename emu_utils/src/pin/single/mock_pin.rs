use crate::pin::{
    PinError, PinSignal, SinglePinCore, SinglePinInterface, SinglePinOutput,
    possible::PossibleSignals,
};
use delegate::delegate;

pub struct MockPin {
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
}

impl SinglePinCore for MockPin {
    fn new(name: String) -> Self {
        Self {
            name,
            signals: PossibleSignals::from(false, false, false),
            prev_signals: PossibleSignals::from(false, false, true),
        }
    }

    fn post_tick_update(&mut self) {
        self.prev_signals = self.signals;
        self.signals.set_all(false);
    }

    fn interface<E>(&self) -> SinglePinInterface<'_, E, Self, false> {
        SinglePinInterface::from_ref(self)
    }

    fn interface_mut<E>(&mut self) -> SinglePinInterface<'_, E, Self, true> {
        SinglePinInterface::from_mut(self)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    delegate! {
        to self.signals{
            #[call(iter_all_enabled)]
            fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn possible_signals(&self) -> Vec<PinSignal>;

            #[call(all_possible_reads)]
            fn possible_reads(&self) -> Vec<bool>;

            fn collapsed(&self) -> Option<PinSignal>;
        }

        to self.prev_signals{
            #[call(iter_all_enabled)]
            fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn prev_possible_signals(&self) -> Vec<PinSignal>;

            #[call(all_possible_reads)]
            fn prev_possible_reads(&self) -> Vec<bool>;

            #[call(collapsed)]
            fn prev_collapsed(&self) -> Option<PinSignal>;
        }

        #[expr($; Ok(()))]
        to self.signals{
            #[call(set_signal)]
            fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError>;

            #[call(set_all)]
            fn set_all_signals_in(&mut self, possible: bool) -> Result<(), PinError>;
        }
    }

    fn set_possible_in_to_prev(&mut self) -> Result<(), PinError> {
        self.signals = self.prev_signals;
        Ok(())
    }
}

impl SinglePinOutput for MockPin {
    delegate! {
        #[expr($; Ok(()))]
        to self.signals{
            #[call(set_signal)]
            fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError>;

            #[call(set_all)]
            fn set_all_signals_out(&mut self, possible: bool) -> Result<(), PinError>;
        }
    }

    fn set_possible_out_to_prev(&mut self) -> Result<(), PinError> {
        self.signals = self.prev_signals;
        Ok(())
    }
}
