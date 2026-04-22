use crate::pin::{PinCore, PinError, PinInputUI, PinOutput, PinSignal, possible::PossibleSignals};

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

    fn update_contention(&mut self) -> Result<(), PinError> {
        let Some(contended_signals) =
            PossibleSignals::contend_together(self.signals_in, self.signals_out)
        else {
            return Err(self.short_circuit_err());
        };
        self.contended_signals = contended_signals;
        Ok(())
    }
}

impl PinCore for ContentionPin {
    fn new<S: Into<String>>(name: S) -> Self {
        let signals_in = PossibleSignals::from(false, false, false);
        let signals_out = PossibleSignals::from(true, true, true);
        let contended_signals = PossibleSignals::contend_together(signals_in, signals_out)
            .expect("this is a valid contention");

        let prev_signals_in = PossibleSignals::from(false, false, true);
        let prev_signals_out = PossibleSignals::from(false, false, true);
        let prev_contended_signals =
            PossibleSignals::contend_together(prev_signals_in, prev_signals_out)
                .expect("this is a valid contention");

        Self {
            name: name.into(),
            signals_in,
            signals_out,
            contended_signals,
            prev_signals_in,
            prev_signals_out,
            prev_contended_signals,
        }
    }

    fn post_tick_update(&mut self) {
        self.prev_signals_in = self.signals_in;
        self.prev_signals_out = self.signals_out;
        self.prev_contended_signals = self.contended_signals;
        self.signals_in.set_all(false, false, false);
        self.signals_out.set_all(false, false, false);
        self.contended_signals.set_all(false, false, false);
    }
}

impl PinInputUI for ContentionPin {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn signal_possible_when(&self, signal: PinSignal, prev: bool) -> bool {
        if prev {
            self.prev_contended_signals.signal_possible(signal)
        } else {
            self.contended_signals.signal_possible(signal)
        }
    }

    fn add_signal_in(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError> {
        self.signals_in.add_signal(signal, only_possible);
        self.update_contention()
    }

    fn remove_signal_in(&mut self, signal: PinSignal) {
        self.signals_in.remove_signal(signal);
        self.update_contention()
            .expect("removing possible signals cannot cause a short-circuit");
    }
}

impl PinOutput for ContentionPin {
    fn add_signal_out(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), PinError> {
        self.signals_out.add_signal(signal, only_possible);
        self.update_contention()
    }

    fn remove_signal_out(&mut self, signal: PinSignal) {
        self.signals_out.remove_signal(signal);
        self.update_contention()
            .expect("removing possible signals cannot cause a short-circuit");
    }
}

#[cfg(test)]
mod tests {
    use crate::pin::PinSignal;

    use super::*;
    use rstest::{fixture, rstest};

    type PinType = ContentionPin;
    const PIN_NAME: &str = "pin";

    #[fixture]
    fn pin_default() -> PinType {
        ContentionPin::new(PIN_NAME)
    }

    #[fixture]
    fn pin_none_out() -> PinType {
        let mut pin = ContentionPin::new(PIN_NAME);
        pin.set_all_out(false, false, false).unwrap();
        pin
    }

    #[fixture]
    fn pin_high_z_out() -> PinType {
        let mut pin = ContentionPin::new(PIN_NAME);
        pin.set_all_out(false, false, true).unwrap();
        pin
    }

    #[rstest]
    fn name(#[from(pin_default)] pin: PinType) {
        assert_eq!(pin.name(), PIN_NAME);
    }

    #[rstest]
    fn initial_state(#[from(pin_default)] pin: PinType) {
        assert_eq!(pin.prev_collapsed().unwrap(), PinSignal::HighZ);
        assert_eq!(pin.iter_possible_signals().count(), 0);
    }

    #[rstest]
    fn post_tick_update(
        #[from(pin_none_out)] mut pin: PinType,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        pin.add_signal_in(signal, false).unwrap();
        pin.add_signal_out(signal, false).unwrap();
        pin.post_tick_update();
        assert_eq!(pin.prev_collapsed().unwrap(), signal);
        assert_eq!(pin.iter_possible_signals().count(), 0);
    }

    #[rstest]
    #[case(PinSignal::HighZ, PinSignal::HighZ, PinSignal::HighZ)]
    #[case(PinSignal::High, PinSignal::HighZ, PinSignal::High)]
    #[case(PinSignal::Low, PinSignal::HighZ, PinSignal::Low)]
    #[case(PinSignal::HighZ, PinSignal::High, PinSignal::High)]
    #[case(PinSignal::HighZ, PinSignal::Low, PinSignal::Low)]
    #[case(PinSignal::High, PinSignal::High, PinSignal::High)]
    #[case(PinSignal::Low, PinSignal::Low, PinSignal::Low)]
    fn safe_two_way_driving(
        #[from(pin_none_out)] mut pin: PinType,
        #[case] istate: PinSignal,
        #[case] ostate: PinSignal,
        #[case] state: PinSignal,
    ) {
        pin.add_signal_in(istate, false).unwrap();
        pin.add_signal_out(ostate, false).unwrap();
        assert_eq!(pin.collapsed().unwrap(), state);
    }
}
