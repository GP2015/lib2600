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
        }
    }
}
