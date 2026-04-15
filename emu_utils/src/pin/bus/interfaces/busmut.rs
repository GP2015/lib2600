use crate::pin::{BusCore, SinglePinCore, SinglePinMut, SinglePinRef};
use delegate::delegate;
use std::marker::PhantomData;

pub struct BusMut<'a, B, P>
where
    B: BusCore<'a, P>,
    P: 'a + SinglePinCore<'a>,
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

    pub fn pin(&'a self, bit: usize) -> Result<SinglePinRef<'a, P>, P::ErrType> {
        let pin = self.inner.pin(bit)?;
        Ok(SinglePinRef::from(pin))
    }

    pub fn pin_mut(&'a mut self, bit: usize) -> Result<SinglePinMut<'a, P>, P::ErrType> {
        let pin = self.inner.pin_mut(bit)?;
        Ok(SinglePinMut::from(pin))
    }

    pub fn iter(&'a self) -> impl Iterator<Item = SinglePinRef<'a, P>> {
        self.inner.iter().map(|pin| SinglePinRef::from(pin))
    }

    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = SinglePinMut<'a, P>> {
        self.inner.iter_mut().map(|pin| SinglePinMut::from(pin))
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
            pub fn add_possible_drive_in(&mut self, val: usize) -> Result<(), P::ErrType>;
            pub fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), P::ErrType>;
        }
    }
}
