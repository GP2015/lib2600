use crate::pin::{PinError, SinglePinCore, possible::PossibleSignals};
use std::marker::PhantomData;

pub struct InputPin<E>
where
    E: From<PinError>,
{
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
    err_type: PhantomData<E>,
}

impl<E> SinglePinCore<'_> for InputPin<E>
where
    E: From<PinError>,
{
    type ErrType = E;

    fn new(name: String) -> Self {
        Self {
            name,
            signals: PossibleSignals::from(false, false, false),
            prev_signals: PossibleSignals::from(false, false, true),
            err_type: PhantomData,
        }
    }

    fn post_tick_update(&mut self) {
        self.prev_signals = self.signals;
        self.signals.set_all(false);
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn high_possible(&self) -> bool {
        self.signals.high
    }

    fn low_possible(&self) -> bool {
        self.signals.low
    }

    fn high_z_possible(&self) -> bool {
        self.signals.high_z
    }

    fn prev_high_possible(&self) -> bool {
        self.prev_signals.high
    }

    fn prev_low_possible(&self) -> bool {
        self.prev_signals.low
    }

    fn prev_high_z_possible(&self) -> bool {
        self.prev_signals.high_z
    }

    fn set_high_in(&mut self, possible: bool) -> Result<(), Self::ErrType> {
        self.signals.high = possible;
        Ok(())
    }

    fn set_low_in(&mut self, possible: bool) -> Result<(), Self::ErrType> {
        self.signals.low = possible;
        Ok(())
    }

    fn set_high_z_in(&mut self, possible: bool) {
        self.signals.high_z = possible;
    }
}

#[cfg(test)]
mod tests {
    use crate::pin::PinSignal;

    use super::*;
    use rstest::{fixture, rstest};

    type PinType = InputPin<PinError>;
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
        pin.set_in(signal, true).unwrap();
        pin.post_tick_update();
        assert_eq!(pin.prev_collapsed().unwrap(), signal);
        assert!(pin.possible_signals().is_empty());
    }
}
