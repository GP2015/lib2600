use crate::pin::{
    PinError, SinglePinCore,
    bus::interfaces::{busmut::BusMut, busref::BusRef},
};

pub trait BusCore<'a, P> {
    fn new(name: String, size: usize) -> Self;
    fn post_tick_update(&mut self);

    fn interface<E>(&'a self) -> BusRef<'a, Self, P, E>
    where
        Self: Sized,
        P: SinglePinCore<'a>,
        E: From<PinError>,
    {
        BusRef::from(self)
    }

    fn interface_mut<E>(&'a mut self) -> BusMut<'a, Self, P, E>
    where
        Self: Sized,
        P: SinglePinCore<'a>,
        E: From<PinError>,
    {
        BusMut::from(self)
    }

    fn for_each_pin_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut P);

    fn name(&self) -> &str;
    fn size(&self) -> usize;
    fn pin(&self, bit: usize) -> Result<&P, PinError>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut P, PinError>;
    fn read(&self) -> Option<usize>;
    fn read_prev(&self) -> Option<usize>;
    fn add_possible_drive_in(&mut self, val: usize) -> Result<(), PinError>;
    fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), PinError>;
}
