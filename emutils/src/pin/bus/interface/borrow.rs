use crate::pin::{BusInputter, PinInputUIBorrow, PinInputter};
use delegate::delegate;
use std::marker::PhantomData;

pub struct BusInputUI<'a, B, P>
where
    B: BusInputter<'a, P>,
    P: 'a + PinInputter<'a>,
{
    inner: &'a B,
    pin_type: PhantomData<P>,
}

impl<'a, B, P> BusRef<'a, B, P>
where
    B: BusInputter<'a, P>,
    P: PinInputter<'a>,
{
    pub(crate) fn from(bus: &'a B) -> Self {
        Self {
            inner: bus,
            pin_type: PhantomData,
        }
    }

    pub fn pin(&'a self, bit: usize) -> Result<impl PinInputUIBorrow, P::ErrType> {
        let pin = self.inner.pin(bit)?;
        Ok(PinInputUIBorrow::from(pin))
    }

    pub fn iter(&'a self) -> impl Iterator<Item = PinInputUIBorrow<'a, P>> {
        self.inner.iter().map(|pin| PinInputUIBorrow::from(pin))
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
    }
}
