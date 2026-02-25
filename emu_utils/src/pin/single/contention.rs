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

        let prev_signals_in = PossibleSignals::from(false, false, true);
        let prev_signals_out = PossibleSignals::from(false, false, true);
        let prev_contended_signals =
            PossibleSignals::contend_together(prev_signals_in, prev_signals_out).unwrap();

        Self {
            name,
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

            #[call(all_possible_reads)]
            fn possible_reads(&self) -> Vec<bool>;

            fn collapsed(&self) -> Option<PinSignal>;
        }

        to self.prev_contended_signals{
            #[call(iter_all_enabled)]
            fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;

            #[call(all_enabled)]
            fn prev_possible_signals(&self) -> Vec<PinSignal>;

            #[call(all_possible_reads)]
            fn prev_possible_reads(&self) -> Vec<bool>;

            #[call(collapsed)]
            fn prev_collapsed(&self) -> Option<PinSignal>;
        }
    }

    fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), PinError> {
        self.update_in(self.signals_in.with_signal(signal, possible))
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

    fn set_all_signals_out(&mut self, possible: bool) -> Result<(), PinError> {
        self.update_out(self.signals_out.with_all(possible))
    }

    fn set_possible_out_to_prev(&mut self) -> Result<(), PinError> {
        self.update_out(self.prev_signals_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = ContentionPin;
    const PIN_NAME: &str = "pin";

    #[fixture]
    fn pin_default() -> PinType {
        ContentionPin::new(String::from(PIN_NAME))
    }

    #[fixture]
    fn pin_none_out() -> PinType {
        let mut pin = ContentionPin::new(String::from(PIN_NAME));
        pin.set_all_signals_out(false).unwrap();
        pin
    }

    #[fixture]
    fn pin_high_z_out() -> PinType {
        let mut pin = ContentionPin::new(String::new());
        pin.set_all_signals_out(false).unwrap();
        pin.add_high_z_out();
        pin
    }

    #[rstest]
    fn name(#[from(pin_default)] pin: PinType) {
        assert_eq!(pin.name(), PIN_NAME);
    }

    #[rstest]
    fn initial_state(#[from(pin_default)] pin: PinType) {
        assert_eq!(pin.prev_collapsed().unwrap(), PinSignal::HighZ);
        assert!(pin.possible_signals().is_empty());
    }

    #[rstest]
    fn post_tick_update(
        #[from(pin_none_out)] mut pin: PinType,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::HighZ)] signal: PinSignal,
    ) {
        pin.add_signal_in(signal).unwrap();
        pin.add_signal_out(signal).unwrap();
        pin.post_tick_update();
        assert_eq!(pin.prev_collapsed().unwrap(), signal);
        assert!(pin.possible_signals().is_empty());
    }

    #[rstest]
    #[case(vec![PinSignal::High])]
    fn possible_signals(#[from(pin_high_z_out)] mut pin: PinType, #[case] signals: Vec<PinSignal>) {
        assert!(!pin.signals_in.high);
        assert!(!pin.signals_in.low);
        assert!(!pin.signals_in.high_z);
        assert!(!pin.signals_out.high);
        assert!(!pin.signals_out.low);
        assert!(pin.signals_out.high_z);
        for signal in &signals {
            pin.add_signal_in(*signal).unwrap();
        }
        assert_eq!(pin.possible_signals(), signals);
        pin.post_tick_update();
        assert_eq!(pin.prev_possible_signals(), signals);
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
        pin.add_signal_in(istate).unwrap();
        pin.add_signal_out(ostate).unwrap();
        assert_eq!(pin.collapsed().unwrap(), state);
    }
}
