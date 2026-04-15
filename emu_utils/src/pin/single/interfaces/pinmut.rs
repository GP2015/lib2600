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
            pub fn possible_signals(&self) -> Vec<PinSignal>;
            pub fn prev_possible_signals(&self) -> Vec<PinSignal>;
            pub fn possible_reads(&self) -> Vec<bool>;
            pub fn prev_possible_reads(&self) -> Vec<bool>;
            pub fn collapsed(&self) -> Option<PinSignal>;
            pub fn prev_collapsed(&self) -> Option<PinSignal>;
        }


        to self.inner {
            pub fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
            pub fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
            pub fn set_high_z_in(&mut self, possible: bool);
            pub fn add_high_z_in(&mut self);
        }

        #[expr($.map_err((Into::into)))]
        to self.inner {
            pub fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), P::ErrType>;
            pub fn set_all_signals_in(&mut self, possible: bool) -> Result<(), P::ErrType>;
            pub fn set_possible_in_to_prev(&mut self) -> Result<(), P::ErrType>;
            pub fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), P::ErrType>;
            pub fn add_signal_in(&mut self, signal: PinSignal) -> Result<(), P::ErrType>;
            pub fn add_drive_in(&mut self, bool_signal: bool) -> Result<(), P::ErrType>;
            pub fn add_all_signals_in(&mut self) -> Result<(), P::ErrType>;
        }
    }
}
