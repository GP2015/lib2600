pub mod standard;

use std::marker::PhantomData;

use delegate::delegate;

use crate::pin::{PinError, SinglePinCore, SinglePinMut, SinglePinRef};

pub trait BusCore<P> {
    fn new(name: String, size: usize) -> Self;
    fn post_tick_update(&mut self);
    fn name(&self) -> &str;
    fn pin(&self, bit: usize) -> Result<&P, PinError>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut P, PinError>;
    fn read(&self) -> Option<usize>;
    fn read_prev(&self) -> Option<usize>;
    fn add_possible_drive_in(&mut self, val: usize) -> Result<(), PinError>;
    fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), PinError>;
}

pub trait BusOutput<P> {
    fn pin_out(&mut self, bit: usize) -> Result<&mut P, PinError>;
    fn add_possible_drive_out(&mut self, val: usize) -> Result<(), PinError>;
    fn add_possible_tri_state_out(&mut self);
    fn remove_all_possible_out(&mut self);
    fn set_all_possible_out_to_prev(&mut self) -> Result<(), PinError>;
}

pub struct BusRef<'a, B, E, P> {
    inner: &'a B,
    err_type: PhantomData<E>,
    pin_type: PhantomData<P>,
}

impl<'a, B, E, P> BusRef<'a, B, E, P>
where
    B: BusCore<P>,
    E: From<PinError>,
    P: SinglePinCore,
{
    pub fn from(bus: &'a B) -> Self {
        Self {
            inner: bus,
            err_type: PhantomData,
            pin_type: PhantomData,
        }
    }

    pub fn pin(&self, bit: usize) -> Result<SinglePinRef<'_, E, P>, E> {
        Ok(SinglePinRef::from(self.inner.pin(bit)?))
    }

    delegate! {
        to self.inner{
            pub fn name(&self) -> &str;
            pub fn read(&self) -> Option<usize>;
            pub fn read_prev(&self) -> Option<usize>;
        }
    }
}

pub struct BusMut<'a, B, E, P>
where
    B: BusCore<P>,
    E: From<PinError>,
    P: SinglePinCore,
{
    inner: &'a mut B,
    err_type: PhantomData<E>,
    pin_type: PhantomData<P>,
}

impl<'a, B, E, P> BusMut<'a, B, E, P>
where
    B: BusCore<P>,
    E: From<PinError>,
    P: SinglePinCore,
{
    pub fn from(bus: &'a mut B) -> Self {
        Self {
            inner: bus,
            err_type: PhantomData,
            pin_type: PhantomData,
        }
    }

    pub fn pin(&self, bit: usize) -> Result<SinglePinRef<'_, E, P>, E> {
        Ok(SinglePinRef::from(self.inner.pin(bit)?))
    }

    pub fn pin_mut(&mut self, bit: usize) -> Result<SinglePinMut<'_, E, P>, E> {
        Ok(SinglePinMut::from(self.inner.pin_mut(bit)?))
    }

    delegate! {
        to self.inner{
            pub fn name(&self) -> &str;
            pub fn read(&self) -> Option<usize>;
            pub fn read_prev(&self) -> Option<usize>;
        }

        #[expr($.map_err(E::from))]
        to self.inner{
            pub fn add_possible_drive_in(&mut self, val: usize) -> Result<(), E>;
            pub fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), E>;
        }
    }
}
