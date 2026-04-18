use crate::pin::{PinCore, PinError, PinInputUI, PinOutput, PinSignal, possible::PossibleSignals};
use delegate::delegate;

pub struct MockPin {
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
}

impl PinCore for MockPin {
    fn new(name: String) -> Self {
        Self {
            name,
            signals: PossibleSignals::from(false, false, false),
            prev_signals: PossibleSignals::from(false, false, true),
        }
    }

    fn post_tick_update(&mut self) {
        self.prev_signals = self.signals;
        self.signals.set_all(false, false, false);
    }
}

impl PinInputUI for MockPin {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    delegate! {
        to self.signals{
            fn signal_possible(&self, signal: PinSignal) -> bool;

            #[call(add_signal)]
            #[expr($; Ok(()))]
            fn add_signal_in(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError>;

            #[call(remove_signal)]
            fn remove_signal_in(&mut self, signal: PinSignal);
        }

        to self.prev_signals{
            #[call(signal_possible)]
            fn prev_signal_possible(&self, signal: PinSignal) -> bool;
        }
    }
}

impl PinOutput for MockPin {
    delegate! {
        to self.signals{
            #[call(add_signal)]
            #[expr($; Ok(()))]
            fn add_signal_out(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError>;

            #[call(remove_signal)]
            fn remove_signal_out(&mut self, signal: PinSignal);
        }
    }
}
