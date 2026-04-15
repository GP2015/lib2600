use crate::pin::{PinError, PinSignal, SinglePinCore};
use delegate::delegate;
use std::marker::PhantomData;

pub struct SinglePinMut<'a, P, E>
where
    P: SinglePinCore<'a>,
    E: From<PinError>,
{
    inner: &'a mut P,
    err_type: PhantomData<E>,
}

impl<'a, P, E> SinglePinMut<'a, P, E>
where
    P: SinglePinCore<'a>,
    E: From<PinError>,
{
    pub(crate) fn from(pin: &'a mut P) -> Self {
        Self {
            inner: pin,
            err_type: PhantomData,
        }
    }

    delegate! {
        #[must_use]
        to self.inner{
            pub fn name(&self) -> &str;
            pub fn possible_signals(&self) -> Vec<PinSignal>;
            pub fn prev_possible_signals(&self) -> Vec<PinSignal>;
            pub fn possible_reads(&self) -> Vec<bool>;
            pub fn prev_possible_reads(&self) -> Vec<bool>;
            pub fn collapsed(&self) -> Option<PinSignal>;
            pub fn prev_collapsed(&self) -> Option<PinSignal>;
        }


        to self.inner{
            pub fn iter_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
            pub fn iter_prev_possible_signals(&self) -> impl Iterator<Item = PinSignal>;
            pub fn set_high_z_in(&mut self, possible: bool);
            pub fn add_high_z_in(&mut self);
        }

        #[expr($.map_err((Into::into)))]
        to self.inner{
            pub fn set_signal_in(&mut self, signal: PinSignal, possible: bool) -> Result<(), E>;
            pub fn set_all_signals_in(&mut self, possible: bool) -> Result<(), E>;
            pub fn set_possible_in_to_prev(&mut self) -> Result<(), E>;
            pub fn set_drive_in(&mut self, bool_signal: bool, possible: bool) -> Result<(), E>;
            pub fn add_signal_in(&mut self, signal: PinSignal) -> Result<(), E>;
            pub fn add_drive_in(&mut self, bool_signal: bool) -> Result<(), E>;
            pub fn add_all_signals_in(&mut self) -> Result<(), E>;
        }
    }
}
