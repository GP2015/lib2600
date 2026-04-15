use crate::pin::{
    PinError, PinSignal,
    single::interfaces::{pinmut::SinglePinMut, pinref::SinglePinRef},
};

pub trait SinglePinCore<'a> {
    type ErrType: From<PinError>;

    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);
    fn name(&self) -> &str;
    fn high_possible(&self) -> bool;
    fn low_possible(&self) -> bool;
    fn high_z_possible(&self) -> bool;
    fn prev_high_possible(&self) -> bool;
    fn prev_low_possible(&self) -> bool;
    fn prev_high_z_possible(&self) -> bool;
    fn set_high_in(&mut self, possible: bool) -> Result<(), Self::ErrType>;
    fn set_low_in(&mut self, possible: bool) -> Result<(), Self::ErrType>;
    fn set_high_z_in(&mut self, possible: bool);

    fn interface(&'a self) -> SinglePinRef<'a, Self>
    where
        Self: Sized,
    {
        SinglePinRef::from(self)
    }

    fn interface_mut(&'a mut self) -> SinglePinMut<'a, Self>
    where
        Self: Sized,
    {
        SinglePinMut::from(self)
    }

    fn add_high_in(&mut self) -> Result<(), Self::ErrType> {
        self.set_high_in(true)
    }

    fn add_low_in(&mut self) -> Result<(), Self::ErrType> {
        self.set_low_in(true)
    }

    fn add_high_z_in(&mut self) {
        self.set_high_z_in(true);
    }

    fn set_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), Self::ErrType> {
        match signal {
            PinSignal::High => self.set_high_in(possible)?,
            PinSignal::Low => self.set_low_in(possible)?,
            PinSignal::HighZ => self.set_high_z_in(possible),
        }
        Ok(())
    }

    fn add_in(&mut self, signal: PinSignal) -> Result<(), Self::ErrType> {
        self.set_in(signal, true)
    }

    fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), Self::ErrType> {
        if bool_signal {
            self.set_high_in(possible)
        } else {
            self.set_low_in(possible)
        }
    }

    fn add_drive_in(&mut self, bool_signal: bool) -> Result<(), Self::ErrType> {
        self.set_drive_in(bool_signal, true)
    }

    fn set_all_in(&mut self, possible: bool) -> Result<(), Self::ErrType> {
        self.set_high_in(possible)?;
        self.set_low_in(possible)?;
        self.set_high_z_in(possible);
        Ok(())
    }

    fn add_all_in(&mut self) -> Result<(), Self::ErrType> {
        self.set_all_in(true)
    }

    fn set_in_to_prev(&mut self) -> Result<(), Self::ErrType> {
        self.set_high_in(self.high_possible())?;
        self.set_low_in(self.low_possible())?;
        self.set_high_z_in(self.high_z_possible());
        Ok(())
    }

    fn could_read_high(&self) -> bool {
        self.high_possible() | self.high_z_possible()
    }

    fn could_read_low(&self) -> bool {
        self.low_possible() | self.high_z_possible()
    }

    fn collapsed(&self) -> Option<PinSignal> {
        match (
            self.high_possible(),
            self.low_possible(),
            self.high_z_possible(),
        ) {
            (true, false, false) => Some(PinSignal::High),
            (false, true, false) => Some(PinSignal::Low),
            (false, false, true) => Some(PinSignal::HighZ),
            _ => None,
        }
    }

    fn prev_collapsed(&self) -> Option<PinSignal> {
        match (
            self.prev_high_possible(),
            self.prev_low_possible(),
            self.prev_high_z_possible(),
        ) {
            (true, false, false) => Some(PinSignal::High),
            (false, true, false) => Some(PinSignal::Low),
            (false, false, true) => Some(PinSignal::HighZ),
            _ => None,
        }
    }

    fn possible_signals(&self) -> Vec<PinSignal> {
        [
            (self.high_possible(), PinSignal::High),
            (self.low_possible(), PinSignal::Low),
            (self.high_z_possible(), PinSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(state, signal)| state.then_some(signal))
        .collect()
    }

    fn prev_possible_signals(&self) -> Vec<PinSignal> {
        [
            (self.prev_high_possible(), PinSignal::High),
            (self.prev_low_possible(), PinSignal::Low),
            (self.prev_high_z_possible(), PinSignal::HighZ),
        ]
        .into_iter()
        .filter_map(|(state, signal)| state.then_some(signal))
        .collect()
    }

    fn possible_reads(&self) -> Vec<bool> {
        match (
            self.high_possible(),
            self.low_possible(),
            self.high_z_possible(),
        ) {
            (false, false, false) => Vec::new(),
            (false, true, false) => vec![false],
            (true, false, false) => vec![true],
            (true, true, false) | (_, _, true) => vec![true, false],
        }
    }

    fn prev_possible_reads(&self) -> Vec<bool> {
        match (
            self.prev_high_possible(),
            self.prev_low_possible(),
            self.prev_high_z_possible(),
        ) {
            (false, false, false) => Vec::new(),
            (false, true, false) => vec![false],
            (true, false, false) => vec![true],
            (true, true, false) | (_, _, true) => vec![true, false],
        }
    }
}
