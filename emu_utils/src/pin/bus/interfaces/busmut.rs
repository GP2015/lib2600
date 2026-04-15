use crate::pin::{BusCore, PinError, SinglePinCore};
use delegate::delegate;
use std::marker::PhantomData;

pub struct BusMut<'a, B, P, E>
where
    B: BusCore<'a, P>,
    P: SinglePinCore<'a>,
    E: From<PinError>,
{
    inner: &'a mut B,
    pin_type: PhantomData<P>,
    err_type: PhantomData<E>,
}

impl<'a, B, P, E> BusMut<'a, B, P, E>
where
    B: BusCore<'a, P>,
    P: SinglePinCore<'a>,
    E: From<PinError>,
{
    pub(crate) fn from(bus: &'a mut B) -> Self {
        Self {
            inner: bus,
            pin_type: PhantomData,
            err_type: PhantomData,
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

        to self.inner{
            pub fn for_each_pin_mut<F>(&mut self, f: F)
            where
                F: FnMut(&mut P);
        }

        #[expr($.map_err(Into::into))]
        to self.inner{
            pub fn pin(&self, bit: usize) -> Result<&P, E>;
            pub fn pin_mut(&mut self, bit: usize) -> Result<&mut P, E>;
            pub fn add_possible_drive_in(&mut self, val: usize) -> Result<(), E>;
            pub fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), E>;
        }
    }
}
