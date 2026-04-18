use crate::pin::{PinCore, PinError, PinInputUI, PinSignal, possible::PossibleSignals};
use delegate::delegate;

pub struct InputPin {
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
}

impl PinCore for InputPin {
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

impl PinInputUI for InputPin {
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

#[cfg(test)]
mod tests {
    use crate::pin::PinSignal;

    use super::*;
    use rstest::{fixture, rstest};

    type PinType = InputPin;
    const PIN_NAME: &str = "pin";

    #[fixture]
    fn pin() -> PinType {
        InputPin::new(String::from(PIN_NAME))
    }

    #[rstest]
    fn name(pin: PinType) {
        assert_eq!(pin.name(), PIN_NAME);
    }

    #[rstest]
    fn initial_state(pin: PinType) {
        assert_eq!(pin.prev_collapsed().unwrap(), PinSignal::HighZ);
        assert!(pin.possible_signals().is_empty());
    }

    #[rstest]
    fn post_tick_update(
        mut pin: PinType,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        pin.add_signal_in(signal, true).unwrap();
        pin.post_tick_update();
        assert_eq!(pin.prev_collapsed().unwrap(), signal);
        assert!(pin.possible_signals().is_empty());
    }
}
