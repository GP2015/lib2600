use crate::pin::{PinCore, PinError, PinInputUI, PinOutput, PinSignal, possible::PossibleSignals};
use delegate::delegate;

pub struct MockPin {
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
}

impl PinCore for MockPin {
    fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
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

    fn signal_possible_when(&self, signal: PinSignal, prev: bool) -> bool {
        if prev {
            self.prev_signals.is_possible(signal)
        } else {
            self.signals.is_possible(signal)
        }
    }

    delegate! {
        to self.signals {
            #[call(add)]
            #[expr($; Ok(()))]
            fn add_signal_in(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError>;

            #[call(remove)]
            fn remove_signal_in(&mut self, signal: PinSignal);
        }
    }
}

impl PinOutput for MockPin {
    delegate! {
        to self.signals {
            #[call(add)]
            #[expr($; Ok(()))]
            fn add_signal_out(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError>;

            #[call(remove)]
            fn remove_signal_out(&mut self, signal: PinSignal);
        }
    }
}
