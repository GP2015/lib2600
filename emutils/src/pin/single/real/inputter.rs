use crate::pin::{PinError, PinInputUIBorrow, PinInputUIMutate, PinSignal};
use std::fmt::Debug;

pub trait PinInputter<'a> {
    type ErrType: From<PinError> + Debug;

    fn new(name: String) -> Self;
    fn post_tick_update(&mut self);
    fn name(&self) -> &str;
    fn signal_possible(&self, signal: PinSignal) -> bool;
    fn prev_signal_possible(&self, signal: PinSignal) -> bool;
    fn add_signal_in(
        &mut self,
        signal: PinSignal,
        only_possible: bool,
    ) -> Result<(), Self::ErrType>;
    fn remove_signal_in(&mut self, signal: PinSignal);

    fn interface(&'a self) -> impl PinInputUIBorrow;
    fn interface_mut(&'a mut self) -> impl PinInputUIMutate;

    fn high_possible(&self) -> bool {
        self.signal_possible(PinSignal::High)
    }

    fn low_possible(&self) -> bool {
        self.signal_possible(PinSignal::Low)
    }

    fn high_z_possible(&self) -> bool {
        self.signal_possible(PinSignal::HighZ)
    }

    fn prev_high_possible(&self) -> bool {
        self.prev_signal_possible(PinSignal::High)
    }

    fn prev_low_possible(&self) -> bool {
        self.prev_signal_possible(PinSignal::Low)
    }

    fn prev_high_z_possible(&self) -> bool {
        self.prev_signal_possible(PinSignal::HighZ)
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

    fn add_high_in(&mut self, only_possible: bool) -> Result<(), Self::ErrType> {
        self.add_signal_in(PinSignal::High, only_possible)
    }

    fn add_low_in(&mut self, only_possible: bool) -> Result<(), Self::ErrType> {
        self.add_signal_in(PinSignal::Low, only_possible)
    }

    fn add_high_z_in(&mut self, only_possible: bool) {
        self.add_signal_in(PinSignal::HighZ, only_possible)
            .expect("setting high impedance in cannot cause a short-circuit");
    }

    fn remove_high_in(&mut self) {
        self.remove_signal_in(PinSignal::High);
    }

    fn remove_low_in(&mut self) {
        self.remove_signal_in(PinSignal::Low);
    }

    fn remove_high_z_in(&mut self) {
        self.remove_signal_in(PinSignal::HighZ);
    }

    fn add_drive_in(
        &mut self,
        bool_signal: bool,
        only_possible: bool,
    ) -> Result<(), Self::ErrType> {
        if bool_signal {
            self.add_high_in(only_possible)
        } else {
            self.add_low_in(only_possible)
        }
    }

    fn remove_drive_in(&mut self, bool_signal: bool) {
        if bool_signal {
            self.remove_high_in();
        } else {
            self.remove_low_in();
        }
    }

    fn set_all_in(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), Self::ErrType> {
        if high {
            self.add_high_in(false)?;
        } else {
            self.remove_high_in();
        }

        if low {
            self.add_low_in(false)?;
        } else {
            self.remove_low_in();
        }

        if high_z {
            self.add_high_z_in(false);
        } else {
            self.remove_high_z_in();
        }

        Ok(())
    }

    fn set_in_to_prev(&mut self) -> Result<(), Self::ErrType> {
        self.set_all_in(
            self.high_possible(),
            self.low_possible(),
            self.high_z_possible(),
        )
    }
}
