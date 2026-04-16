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

    fn name(&self) -> &str;
    fn size(&self) -> usize;
    fn pin(&self, bit: usize) -> Result<&P, P::ErrType>;
    fn pin_mut(&mut self, bit: usize) -> Result<&mut P, P::ErrType>;
    fn iter(&'a self) -> impl Iterator<Item = &'a P>;
    fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut P>;
    fn read(&self) -> Option<usize>;
    fn read_prev(&self) -> Option<usize>;
    fn only_one_possible_read(&self) -> bool;
    fn iter_possible_reads(&self) -> impl Iterator<Item = usize>;
    fn iter_prev_possible_reads(&self) -> impl Iterator<Item = usize>;
    fn add_drive_in(&mut self, val: usize, only_possible: bool) -> Result<(), P::ErrType>;
    fn add_drive_in_wrapping(&mut self, val: usize, only_possible: bool) -> Result<(), P::ErrType>;
}
