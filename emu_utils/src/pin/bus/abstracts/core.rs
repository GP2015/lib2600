use crate::pin::{
    SinglePinCore,
    bus::interfaces::{busmut::BusMut, busref::BusRef},
};

pub trait BusCore<'a, P>
where
    P: 'a + SinglePinCore<'a>,
{
    fn new(name: String, size: usize) -> Self;
    fn post_tick_update(&mut self);

    fn interface(&'a self) -> BusRef<'a, Self, P>
    where
        Self: Sized,
    {
        BusRef::from(self)
    }

    fn interface_mut(&'a mut self) -> BusMut<'a, Self, P>
    where
        Self: Sized,
    {
        BusMut::from(self)
    }

    fn iter(&'a self) -> impl Iterator<Item = &'a P>;
    fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut P>;

    fn name(&self) -> &str;
    fn size(&self) -> usize;
    fn pin(&self, bit: usize) -> Result<&P, P::ErrType>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut P, P::ErrType>;
    fn read(&self) -> Option<usize>;
    fn read_prev(&self) -> Option<usize>;
    fn add_possible_drive_in(&mut self, val: usize) -> Result<(), P::ErrType>;
    fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), P::ErrType>;
}
