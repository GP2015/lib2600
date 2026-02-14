use delegate::delegate;

use crate::pin::{
    PinError, PinSignal, SinglePinCore, SinglePinInterface, possible::PossibleSignals,
};
use std::marker::PhantomData;

pub struct InputPin<E> {
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
    err_type: PhantomData<E>,
}

impl<E> SinglePinCore for InputPin<E> {
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

impl<E: From<PinError>> SinglePinInterface<E> for InputPin<E> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = InputPin<PinError>;

    #[fixture]
    fn pin() -> PinType {
        InputPin::new(String::new())
    }

    #[rstest]
    fn initial_state(pin: PinType) {
        assert_eq!(pin.prev_possible_signals(), vec![PinSignal::TriState]);
        assert_eq!(pin.possible_signals(), vec![]);
    }

    // #[rstest]
    // fn add_possible_signal_in(
    //     mut pin: PinType,
    //     #[values(PinSignal::High, PinSignal::Low, PinSignal::TriState)] state: PinSignal,
    // ) {
    //     pin.signal_in(state).unwrap();
    //     assert_eq!(pin.state(), state);
    // }

    // #[rstest]
    // fn drive_in(mut pin: PinType, #[values(true, false)] b: bool) {
    //     pin.drive_in(b).unwrap();
    //     assert_eq!(pin.state(), PinSignal::from_bool(b));
    // }

    // #[rstest]
    // fn tri_state_in(mut pin: PinType) {
    //     pin.tri_state_in();
    //     assert_eq!(pin.state(), PinSignal::TriState);
    // }
}
