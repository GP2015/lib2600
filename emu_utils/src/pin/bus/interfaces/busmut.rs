use crate::pin::{BusCore, SinglePinCore};
use delegate::delegate;
use std::marker::PhantomData;

pub struct BusMut<'a, B, P>
where
    B: BusCore<'a, P>,
    P: SinglePinCore<'a>,
{
    inner: &'a mut B,
    pin_type: PhantomData<P>,
}

impl<'a, B, P> BusMut<'a, B, P>
where
    B: BusCore<'a, P>,
    P: SinglePinCore<'a>,
{
    pub(crate) fn from(bus: &'a mut B) -> Self {
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

        to self.inner{
            pub fn for_each_pin_mut<F>(&mut self, f: F)
            where
                F: FnMut(&mut P);
        }

        #[expr($.map_err(Into::into))]
        to self.inner{
            pub fn pin(&self, bit: usize) -> Result<&P, P::ErrType>;
            pub fn pin_mut(&mut self, bit: usize) -> Result<&mut P, P::ErrType>;
            pub fn add_possible_drive_in(&mut self, val: usize) -> Result<(), P::ErrType>;
            pub fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), P::ErrType>;
        }
    }
}
