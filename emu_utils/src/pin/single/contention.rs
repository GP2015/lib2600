use crate::pin::{
    PinError, PinSignal, SinglePinCore, SinglePinInterface, SinglePinOutput,
    possible::PossibleSignals,
};
use delegate::delegate;

pub struct ContentionPin {
    name: String,
    signals_in: PossibleSignals,
    signals_out: PossibleSignals,
    contended_signals: PossibleSignals,
    prev_signals_in: PossibleSignals,
    prev_signals_out: PossibleSignals,
    prev_contended_signals: PossibleSignals,
}

impl ContentionPin {
    fn short_circuit_err(&self) -> PinError {
        PinError::ShortCircuit {
            name: self.name.clone(),
        }
    }

    fn handle_contention(
        &mut self,
        signals_in: PossibleSignals,
        signals_out: PossibleSignals,
    ) -> Result<(), PinError> {
        let Some(contended_signals) = PossibleSignals::contend_together(signals_in, signals_out)
        else {
            return Err(self.short_circuit_err());
        };

        self.signals_in = signals_in;
        self.signals_out = signals_out;
        self.contended_signals = contended_signals;
        Ok(())
    }

    fn update_in(&mut self, signals_in: PossibleSignals) -> Result<(), PinError> {
        self.handle_contention(signals_in, self.signals_out)
    }

    fn update_out(&mut self, signals_out: PossibleSignals) -> Result<(), PinError> {
        self.handle_contention(self.signals_in, signals_out)
    }
}

impl SinglePinCore for ContentionPin {
    fn new(name: String) -> Self {
        let signals_in = PossibleSignals::from(false, false, false);
        let signals_out = PossibleSignals::from(true, true, true);
        let contended_signals = PossibleSignals::contend_together(signals_in, signals_out).unwrap();

        Self {
            name,
            signals_in,
            signals_out,
            contended_signals,
            prev_signals_in: PossibleSignals::from(false, false, true),
            prev_signals_out: PossibleSignals::from(false, false, true),
            prev_contended_signals: PossibleSignals::from(false, false, true),
        }
    }

    fn post_tick_update(&mut self) {
        self.prev_signals_in = self.signals_in;
        self.prev_signals_out = self.signals_out;
        self.prev_contended_signals = self.contended_signals;
        self.signals_in.set_all(false);
        self.signals_out.set_all(false);
        self.contended_signals.set_all(false);
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
        to self.contended_signals{
            #[call(iter_all_enabled)]
            fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn possible_signals(&self) -> Vec<PinSignal>;

            fn collapsed(&self) -> Option<PinSignal>;
        }

        to self.prev_contended_signals{
            #[call(iter_all_enabled)]
            fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn prev_possible_signals(&self) -> Vec<PinSignal>;

            #[call(collapsed)]
            fn prev_collapsed(&self) -> Option<PinSignal>;
        }
    }

    fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError> {
        self.update_in(self.signals_in.with_signal(signal, possible))
    }

    fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), PinError> {
        self.update_in(self.signals_in.with_bool_signal(bool_signal, possible))
    }

    fn set_tri_state_in(&mut self, possible: bool) {
        self.update_in(self.signals_in.with_tri_state(possible))
            .unwrap();
    }

    fn set_all_signals_in(&mut self, possible: bool) -> Result<(), PinError> {
        self.update_in(self.signals_in.with_all(possible))
    }

    fn set_possible_in_to_prev(&mut self) -> Result<(), PinError> {
        self.update_in(self.prev_signals_in)
    }
}

impl SinglePinOutput for ContentionPin {
    fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError> {
        self.update_out(self.signals_out.with_signal(signal, possible))
    }

    fn set_drive_out(&mut self, bool_signal: bool, possible: bool) -> Result<(), PinError> {
        self.update_out(self.signals_out.with_bool_signal(bool_signal, possible))
    }

    fn set_tri_state_out(&mut self, possible: bool) {
        self.update_out(self.signals_out.with_tri_state(possible))
            .unwrap();
    }

    fn set_all_signals_out(&mut self, possible: bool) -> Result<(), PinError> {
        self.update_out(self.signals_out.with_all(possible))
    }

    fn set_possible_out_to_prev(&mut self) -> Result<(), PinError> {
        self.update_out(self.prev_signals_out)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::{fixture, rstest};

//     type PinType = ContentionPin;

//     #[fixture]
//     fn pin_default() -> PinType {
//         ContentionPin::new(String::new())
//     }

//     #[rstest]
//     fn initial_state(#[from(pin_default)] pin: PinType) {
//         assert_eq!(pin.state(), PinState::Undefined);
//     }

//     #[fixture]
//     fn pin_tri_state_out() -> PinType {
//         let mut pin = ContentionPin::new(String::new());
//         pin.tri_state_out();
//         pin
//     }

//     type EmptyRes = Result<(), PinError>;
//     type PinSignal = fn(&mut PinType, state: PinState) -> EmptyRes;
//     type Drive = fn(&mut PinType, state: bool) -> EmptyRes;
//     type TriState = fn(&mut PinType);
//     type Undefine = fn(&mut PinType) -> EmptyRes;

//     #[rstest]
//     fn signal(
//         #[from(pin_tri_state_out)] mut pin: PinType,
//         #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
//         state: PinState,
//         #[values(ContentionPin::signal_in, ContentionPin::signal_out)] func: PinSignal,
//     ) {
//         func(&mut pin, state).unwrap();
//         assert_eq!(pin.state(), state);
//     }

//     #[rstest]
//     fn drive(
//         #[from(pin_tri_state_out)] mut pin: PinType,
//         #[values(true, false)] b: bool,
//         #[values(ContentionPin::drive_in, ContentionPin::drive_out)] func: Drive,
//     ) {
//         func(&mut pin, b).unwrap();
//         assert_eq!(pin.state(), PinState::from_bool(b));
//     }

//     #[rstest]
//     fn tri_state(
//         #[from(pin_tri_state_out)] mut pin: PinType,
//         #[values(ContentionPin::tri_state_in, ContentionPin::tri_state_out)] func: TriState,
//     ) {
//         func(&mut pin);
//         assert_eq!(pin.state(), PinState::TriState);
//     }

//     #[rstest]
//     fn undefine(
//         #[from(pin_tri_state_out)] mut pin: PinType,
//         #[values(ContentionPin::undefine_in, ContentionPin::undefine_out)] func: Undefine,
//     ) {
//         func(&mut pin).unwrap();
//         assert_eq!(pin.state(), PinState::Undefined);
//     }

//     #[rstest]
//     #[case(PinState::TriState, PinState::TriState, PinState::TriState)]
//     #[case(PinState::High, PinState::TriState, PinState::High)]
//     #[case(PinState::Low, PinState::TriState, PinState::Low)]
//     #[case(PinState::TriState, PinState::High, PinState::High)]
//     #[case(PinState::TriState, PinState::Low, PinState::Low)]
//     #[case(PinState::High, PinState::High, PinState::High)]
//     #[case(PinState::Low, PinState::Low, PinState::Low)]
//     fn safe_two_way_driving(
//         #[from(pin_tri_state_out)] mut pin: PinType,
//         #[case] istate: PinState,
//         #[case] ostate: PinState,
//         #[case] state: PinState,
//     ) {
//         pin.signal_in(istate).unwrap();
//         pin.signal_out(ostate).unwrap();
//         assert_eq!(pin.state(), state);
//     }

//     #[rstest]
//     fn contention_swap(#[from(pin_default)] mut pin: PinType, #[values(true, false)] state: bool) {
//         pin.drive_out(state).unwrap();
//         pin.drive_in(state).unwrap();
//         assert_eq!(pin.read().unwrap(), state);
//         pin.tri_state_out();
//         pin.drive_in(!state).unwrap();
//         pin.drive_out(!state).unwrap();
//         assert_eq!(pin.read().unwrap(), !state);
//     }

//     #[rstest]
//     fn short_circuit(
//         #[from(pin_tri_state_out)] mut pin: PinType,
//         #[values(true, false)] state: bool,
//         #[values(true, false)] dir: bool,
//     ) {
//         if dir {
//             pin.drive_in(state).unwrap();
//         } else {
//             pin.drive_out(state).unwrap();
//         }

//         assert!(matches!(
//             if dir {
//                 pin.drive_out(!state).err().unwrap()
//             } else {
//                 pin.drive_in(!state).err().unwrap()
//             },
//             PinError::ShortCircuit { .. }
//         ));
//     }

//     #[rstest]
//     #[case(PinState::High, PinState::Undefined)]
//     #[case(PinState::Low, PinState::Undefined)]
//     #[case(PinState::Undefined, PinState::High)]
//     #[case(PinState::Undefined, PinState::Low)]
//     #[case(PinState::Undefined, PinState::Undefined)]
//     fn potential_short_circuit(
//         #[from(pin_tri_state_out)] mut pin: PinType,
//         #[case] istate: PinState,
//         #[case] ostate: PinState,
//         #[values(true, false)] dir: bool,
//     ) {
//         if dir {
//             pin.signal_in(istate).unwrap();
//         } else {
//             pin.signal_out(istate).unwrap();
//         }

//         assert!(matches!(
//             if dir {
//                 pin.signal_out(ostate).err().unwrap()
//             } else {
//                 pin.signal_in(ostate).err().unwrap()
//             },
//             PinError::PotentialShortCircuit { .. }
//         ));
//     }
// }
