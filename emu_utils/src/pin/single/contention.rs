use crate::pin::{
    PinError, PinSignal, SinglePinCore, SinglePinInterface, SinglePinOutput,
    possible::PossibleSignals,
};
use std::{fmt::Debug, marker::PhantomData};

pub struct ContentionPin<E> {
    name: String,
    contended_signals: PossibleSignals,
    signals_in: PossibleSignals,
    signals_out: PossibleSignals,
    prev_signals: PossibleSignals,
    err_type: PhantomData<E>,
}

impl<E: From<PinError>> ContentionPin<E> {
    fn short_circuit_err(&self) -> E {
        E::from(PinError::ShortCircuit {
            name: self.name.clone(),
        })
    }

    fn handle_contention(
        &mut self,
        signals_in: PossibleSignals,
        signals_out: PossibleSignals,
    ) -> Result<(), E> {
        let Some(contended_signals) = PossibleSignals::contend_together(signals_in, signals_out)
        else {
            return Err(self.short_circuit_err());
        };

        self.contended_signals = contended_signals;
        self.signals_in = signals_in;
        self.signals_out = signals_out;
        Ok(())
    }

    fn update_in<F>(&mut self, f: F) -> Result<(), E>
    where
        F: FnOnce(PossibleSignals) -> PossibleSignals,
    {
        self.handle_contention(f(self.signals_in), self.signals_out)
    }

    fn update_out<F>(&mut self, f: F) -> Result<(), E>
    where
        F: FnOnce(PossibleSignals) -> PossibleSignals,
    {
        self.handle_contention(self.signals_in, f(self.signals_out))
    }
}

impl<E> SinglePinCore for ContentionPin<E> {
    fn new(name: String) -> Self {
        let signals_in = PossibleSignals::from(false, false, false);
        let signals_out = PossibleSignals::from(true, true, true);
        let contended_signals = PossibleSignals::contend_together(signals_in, signals_out).unwrap();

        Self {
            name,
            signals_in,
            signals_out,
            contended_signals,
            prev_signals: PossibleSignals::from(false, false, true),
            err_type: PhantomData,
        }
    }

    fn post_tick_update(&mut self) {
        self.prev_signals = self.contended_signals;
        self.signals_in.set_all(false);
        self.signals_out.set_all(false);
    }
}

impl<E: From<PinError> + Debug> SinglePinInterface<E> for ContentionPin<E> {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn possible_signals(&self) -> Vec<PinSignal> {
        self.contended_signals.all_enabled()
    }

    fn prev_possible_signals(&self) -> Vec<PinSignal> {
        self.prev_signals.all_enabled()
    }

    fn collapsed(&self) -> Option<PinSignal> {
        self.contended_signals.collapsed()
    }

    fn prev_collapsed(&self) -> Option<PinSignal> {
        self.prev_signals.collapsed()
    }

    fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), E> {
        self.update_in(|s| s.with_signal(signal, possible))
    }

    fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), E> {
        self.update_in(|s| s.with_bool_signal(bool_signal, possible))
    }

    fn set_tri_state_in(&mut self, possible: bool) {
        self.update_in(|s| s.with_tri_state(possible)).unwrap();
    }

    fn set_all_signals_in(&mut self, possible: bool) -> Result<(), E> {
        self.update_in(|s| s.with_all(possible))
    }
}

impl<E: From<PinError> + Debug> SinglePinOutput<E> for ContentionPin<E> {
    fn set_signal_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), E> {
        self.update_out(|s| s.with_signal(signal, possible))
    }

    fn set_drive_out(&mut self, bool_signal: bool, possible: bool) -> Result<(), E> {
        self.update_out(|s| s.with_bool_signal(bool_signal, possible))
    }

    fn set_tri_state_out(&mut self, possible: bool) {
        self.update_out(|s| s.with_tri_state(possible)).unwrap();
    }

    fn set_all_signals_out(&mut self, possible: bool) -> Result<(), E> {
        self.update_out(|s| s.with_all(possible))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::{fixture, rstest};

//     type PinType = ContentionPin<PinError>;

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
