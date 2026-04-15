use crate::pin::{PinSignal, SinglePinCore};
use delegate::delegate;

pub struct SinglePinRef<'a, P>
where
    P: SinglePinCore<'a>,
{
    inner: &'a P,
}

impl<'a, P> SinglePinRef<'a, P>
where
    P: SinglePinCore<'a>,
{
    pub(crate) fn from(pin: &'a P) -> Self {
        Self { inner: pin }
    }

    delegate! {
        #[must_use]
        to self.inner {
            pub fn name(&self) -> &str;
            pub fn high_possible(&self) -> bool;
            pub fn low_possible(&self) -> bool;
            pub fn high_z_possible(&self) -> bool;
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
    }
}
