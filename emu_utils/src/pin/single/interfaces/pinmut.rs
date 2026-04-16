use crate::pin::{PinSignal, SinglePinCore};
use delegate::delegate;

pub struct SinglePinMut<'a, P>
where
    P: SinglePinCore<'a>,
{
    inner: &'a mut P,
}

impl<'a, P> SinglePinMut<'a, P>
where
    P: SinglePinCore<'a>,
{
    pub(crate) fn from(pin: &'a mut P) -> Self {
        Self { inner: pin }
    }

    delegate! {
        #[must_use]
        to self.inner {
            pub fn name(&self) -> &str;
            pub fn signal_possible(&self, signal: PinSignal) -> bool;
            pub fn high_possible(&self) -> bool;
            pub fn low_possible(&self) -> bool;
            pub fn high_z_possible(&self) -> bool;
            pub fn prev_signal_possible(&self, signal: PinSignal) -> bool;
            pub fn prev_high_possible(&self) -> bool;
            pub fn prev_low_possible(&self) -> bool;
            pub fn prev_high_z_possible(&self) -> bool;
            pub fn could_read_high(&self) -> bool;
            pub fn could_read_low(&self) -> bool;
            pub fn collapsed(&self) -> Option<PinSignal>;
            pub fn prev_collapsed(&self) -> Option<PinSignal>;
            pub fn possible_signals(&self) -> Vec<PinSignal>;
            pub fn prev_possible_signals(&self) -> Vec<PinSignal>;
            pub fn possible_reads(&self) -> Vec<bool>;
            pub fn prev_possible_reads(&self) -> Vec<bool>;
        }

        to self.inner {
            pub fn add_signal_in(&mut self, signal: PinSignal, only_possible: bool) -> Result<(), P::ErrType>;
            pub fn add_high_in(&mut self, only_possible: bool) -> Result<(), P::ErrType>;
            pub fn add_low_in(&mut self, only_possible: bool) -> Result<(), P::ErrType>;
            pub fn add_high_z_in(&mut self, only_possible: bool);
            pub fn remove_signal_in(&mut self, signal: PinSignal);
            pub fn remove_high_in(&mut self);
            pub fn remove_low_in(&mut self);
            pub fn remove_high_z_in(&mut self);
            pub fn add_drive_in(&mut self, bool_signal: bool, only_possible: bool) -> Result<(), P::ErrType>;
            pub fn remove_drive_in(&mut self, bool_signal: bool);
            pub fn set_all_in(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), P::ErrType>;
            pub fn set_in_to_prev(&mut self) -> Result<(), P::ErrType>;
        }
    }
}
