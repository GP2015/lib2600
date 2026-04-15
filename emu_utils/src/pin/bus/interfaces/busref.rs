use crate::pin::{BusCore, SinglePinCore};
use delegate::delegate;
use std::marker::PhantomData;

pub struct BusRef<'a, B, P>
where
    B: BusCore<'a, P>,
    P: SinglePinCore<'a>,
{
    inner: &'a B,
    pin_type: PhantomData<P>,
}

impl<'a, B, P> BusRef<'a, B, P>
where
    B: BusCore<'a, P>,
    P: SinglePinCore<'a>,
{
    pub(crate) fn from(bus: &'a B) -> Self {
        Self {
            inner: bus,
            pin_type: PhantomData,
        }
    }

    delegate! {
        #[must_use]
        to self.inner{
            pub fn name(&self) -> &str;
            pub fn size(&self) -> usize;
            pub fn read(&self) -> Option<usize>;
            pub fn read_prev(&self) -> Option<usize>;
        }

        #[expr($.map_err(Into::into))]
        to self.inner{
            pub fn pin(&self, bit: usize) -> Result<&P, P::ErrType>;
        }
    }
}
