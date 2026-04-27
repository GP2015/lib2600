use crate::pin::{PinCore, PinError, PinInputUI, PinSignal, possible::PossibleSignals};
use delegate::delegate;

pub struct InputPin {
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
}

impl PinCore for InputPin {
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

impl PinInputUI for InputPin {
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

#[cfg(test)]
mod tests {
    use crate::pin::PinSignal;

    use super::*;
    use rstest::{fixture, rstest};

    type PinType = InputPin;
    const PIN_NAME: &str = "pin";

    #[fixture]
    fn pin() -> PinType {
        InputPin::new(PIN_NAME)
    }

    #[rstest]
    fn name(pin: PinType) {
        assert_eq!(pin.name(), PIN_NAME);
    }

    #[rstest]
    fn initial_state(pin: PinType) {
        assert_eq!(pin.prev_collapsed().unwrap(), PinSignal::HighZ);
        assert_eq!(pin.iter_possible_signals().count(), 0);
    }

    #[rstest]
    fn post_tick_update(
        mut pin: PinType,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        pin.add_signal_in(signal, true).unwrap();
        pin.post_tick_update();
        assert_eq!(pin.prev_collapsed().unwrap(), signal);
        assert_eq!(pin.iter_possible_signals().count(), 0);
    }
}
