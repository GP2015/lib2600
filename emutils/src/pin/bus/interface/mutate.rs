use crate::pin::{BusInputter, PinInputUIBorrow, PinInputUIMutate, PinInputter};
use delegate::delegate;
use std::marker::PhantomData;

pub struct BusMut<'a, B, P>
where
    B: BusInputter<'a, P>,
    P: 'a + PinInputter<'a>,
{
    inner: &'a mut B,
    pin_type: PhantomData<P>,
}

impl<'a, B, P> BusMut<'a, B, P>
where
    B: BusInputter<'a, P>,
    P: PinInputter<'a>,
{
    pub(crate) fn from(bus: &'a mut B) -> Self {
        Self {
            inner: bus,
            pin_type: PhantomData,
        }
    }

    pub fn pin(&'a self, bit: usize) -> Result<PinInputUIBorrow<'a, P>, P::ErrType> {
        let pin = self.inner.pin(bit)?;
        Ok(PinInputUIBorrow::from(pin))
    }

    pub fn pin_mut(&'a mut self, bit: usize) -> Result<PinInputUIMutate<'a, P>, P::ErrType> {
        let pin = self.inner.pin_mut(bit)?;
        Ok(PinInputUIMutate::from(pin))
    }

    pub fn iter(&'a self) -> impl Iterator<Item = PinInputUIBorrow<'a, P>> {
        self.inner.iter().map(|pin| PinInputUIBorrow::from(pin))
    }

    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = PinInputUIMutate<'a, P>> {
        self.inner.iter_mut().map(|pin| PinInputUIMutate::from(pin))
    }

    delegate! {
        #[must_use]
        to self.inner {
            pub fn name(&self) -> &str;
            pub fn size(&self) -> usize;
            pub fn read(&self) -> Option<usize>;
            pub fn read_prev(&self) -> Option<usize>;
        }

        to self.inner {
            pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize>;
            pub fn iter_prev_possible_reads(&self) -> impl Iterator<Item = usize>;
        }

        #[expr($.map_err(Into::into))]
        to self.inner {
            pub fn add_drive_in(&mut self, val: usize, only_possible: bool) -> Result<(), P::ErrType>;
            pub fn add_drive_in_wrapping(&mut self, val: usize, only_possible: bool) -> Result<(), P::ErrType>;
        }
    }
}
