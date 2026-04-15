use crate::pin::{BusCore, PinError, SinglePinCore};
use delegate::delegate;
use std::marker::PhantomData;

pub struct BusRef<'a, B, P, E>
where
    B: BusCore<'a, P>,
    P: SinglePinCore<'a>,
    E: From<PinError>,
{
    inner: &'a B,
    pin_type: PhantomData<P>,
    err_type: PhantomData<E>,
}

impl<'a, B, P, E> BusRef<'a, B, P, E>
where
    B: BusCore<'a, P>,
    P: SinglePinCore<'a>,
    E: From<PinError>,
{
    pub(crate) fn from(bus: &'a B) -> Self {
        Self {
            inner: bus,
            err_type: PhantomData,
            pin_type: PhantomData,
        }
    }

    delegate! {
        to self.inner{
            pub fn name(&self) -> &str;
            pub fn size(&self) -> usize;
            pub fn read(&self) -> Option<usize>;
            pub fn read_prev(&self) -> Option<usize>;
        }

        #[expr($.map_err(Into::into))]
        to self.inner{
            pub fn pin(&self, bit: usize) -> Result<&P, E>;
        }
    }
}
