use crate::pin::{PinError, SinglePinCore, SinglePinOutput, possible::PossibleSignals};
use std::marker::PhantomData;

pub struct MockPin<E>
where
    E: From<PinError>,
{
    name: String,
    signals: PossibleSignals,
    prev_signals: PossibleSignals,
    err_type: PhantomData<E>,
}

impl<E> SinglePinCore<'_> for MockPin<E>
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

impl<E> SinglePinOutput<'_> for MockPin<E>
where
    E: From<PinError>,
{
    fn set_high_out(&mut self, possible: bool) -> Result<(), Self::ErrType> {
        self.signals.high = possible;
        Ok(())
    }

    fn set_low_out(&mut self, possible: bool) -> Result<(), Self::ErrType> {
        self.signals.low = possible;
        Ok(())
    }

    fn set_high_z_out(&mut self, possible: bool) {
        self.signals.high_z = possible;
    }
}
