pub mod standard;

use std::marker::PhantomData;

use delegate::delegate;

use crate::pin::{
    PinError, SinglePinCore, SinglePinInterface,
    pin_ref::{Immutable, Mutable, ObjRef, RefType},
};

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

pub struct BusInterface<'a, B, E, M, P> {
    inner: ObjRef<'a, B>,
    err_type: PhantomData<E>,
    ref_type: PhantomData<M>,
    pin_type: PhantomData<P>,
}

impl<'a, B, E, P> BusInterface<'a, B, E, Immutable, P> {
    pub fn from_ref(bus: &'a B) -> Self {
        Self {
            inner: ObjRef::Immutable(bus),
            err_type: PhantomData,
            ref_type: PhantomData,
            pin_type: PhantomData,
        }
    }
}

impl<'a, B, E, P> BusInterface<'a, B, E, Mutable, P> {
    pub fn from_mut(bus: &'a mut B) -> Self {
        Self {
            inner: ObjRef::Mutable(bus),
            err_type: PhantomData,
            ref_type: PhantomData,
            pin_type: PhantomData,
        }
    }
}

impl<'a, B, E, M, P> BusInterface<'a, B, E, M, P>
where
    B: BusCore<P>,
    E: From<PinError>,
    M: RefType,
    P: SinglePinCore,
{
    pub fn pin(&self, bit: usize) -> Result<SinglePinInterface<'_, E, Immutable, P>, E> {
        let pin = self.inner.as_ref().pin(bit)?;
        let iface = SinglePinInterface::from_ref(pin);
        Ok(iface)
    }

    delegate! {
        to self.inner.as_ref(){
            pub fn name(&self) -> &str;
            pub fn read(&self) -> Option<usize>;
            pub fn read_prev(&self) -> Option<usize>;
        }
    }
}

impl<'a, B, E, P> BusInterface<'a, B, E, Mutable, P>
where
    B: BusCore<P>,
    E: From<PinError>,
    P: SinglePinCore,
{
    pub fn pin_mut(&mut self, bit: usize) -> Result<SinglePinInterface<'_, E, Mutable, P>, E> {
        let pin = self.inner.as_mut().unwrap().pin_mut(bit)?;
        let iface = SinglePinInterface::from_mut(pin);
        Ok(iface)
    }

    delegate! {
        #[expr($.map_err(E::from))]
        to self.inner.as_mut().unwrap(){
            pub fn add_possible_drive_in(&mut self, val: usize) -> Result<(), E>;
            pub fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), E>;
        }
    }
}
