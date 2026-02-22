use crate::pin::{
    PinError, PinSignal, SinglePinCore, SinglePinInterface, possible::PossibleSignals,
};
use delegate::delegate;

pub struct InputPin {
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
}

impl SinglePinCore for InputPin {
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
            fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError>;

            #[call(set_bool_signal)]
            fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), PinError>;

            #[call(set_all)]
            fn set_all_signals_in(&mut self, possible: bool) -> Result<(), PinError>;
        }
    }

    fn set_tri_state_in(&mut self, possible: bool) {
        self.signals.tri_state = possible;
    }

    fn set_possible_in_to_prev(&mut self) -> Result<(), PinError> {
        self.signals = self.prev_signals;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(pin.prev_collapsed().unwrap(), PinSignal::TriState);
        assert!(pin.possible_signals().is_empty());
    }

    #[rstest]
    fn post_tick_update(
        mut pin: PinType,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::TriState)] signal: PinSignal,
    ) {
        pin.set_signal_in(signal, true).unwrap();
        pin.post_tick_update();
        assert_eq!(pin.prev_collapsed().unwrap(), signal);
        assert!(pin.possible_signals().is_empty());
    }

    #[rstest]
    #[case(vec![PinSignal::High, PinSignal::Low])]
    fn set_signal_in_and_possible_signals(mut pin: PinType, #[case] signals: Vec<PinSignal>) {
        for signal in &signals {
            pin.set_signal_in(*signal, true).unwrap();
        }
        assert_eq!(pin.possible_signals(), signals);
        pin.post_tick_update();
        assert_eq!(pin.prev_possible_signals(), signals);
    }

    #[rstest]
    fn collapsed(
        mut pin: PinType,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::TriState)] signal: PinSignal,
    ) {
        pin.set_signal_in(signal, true).unwrap();
        assert_eq!(pin.collapsed().unwrap(), signal);
        pin.post_tick_update();
        assert_eq!(pin.prev_collapsed().unwrap(), signal);
    }

    #[rstest]
    #[case(true, PinSignal::High)]
    #[case(false, PinSignal::Low)]
    fn set_drive_in(mut pin: PinType, #[case] bool_signal: bool, #[case] signal: PinSignal) {
        pin.set_drive_in(bool_signal, true).unwrap();
        assert_eq!(pin.collapsed().unwrap(), signal);
    }

    #[rstest]
    fn set_tristate_in(mut pin: PinType) {
        pin.set_tri_state_in(true);
        assert_eq!(pin.collapsed().unwrap(), PinSignal::TriState);
    }

    #[rstest]
    fn set_all_signals_in(mut pin: PinType) {
        pin.set_all_signals_in(true).unwrap();
        assert_eq!(
            pin.possible_signals(),
            vec![PinSignal::High, PinSignal::Low, PinSignal::TriState]
        );
    }

    #[rstest]
    fn set_possible_in_to_prev(
        mut pin: PinType,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::TriState)] signal: PinSignal,
    ) {
        pin.set_signal_in(signal, true).unwrap();
        pin.post_tick_update();
        pin.set_possible_in_to_prev().unwrap();
        assert_eq!(pin.collapsed().unwrap(), signal);
    }
}
