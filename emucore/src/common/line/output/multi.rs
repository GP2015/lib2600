use crate::common::{
    bit::multi::BitStringState, combine::Combine, line::output::single::LineOutput,
};
use core::array;
use derive_more::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, Index, IndexMut, PartialEq)]
pub struct BusOutput<const SIZE: usize>([LineOutput; SIZE]);

impl<const SIZE: usize> Default for BusOutput<SIZE> {
    fn default() -> Self {
        [LineOutput::default(); _].into()
    }
}

impl<const SIZE: usize> From<u16> for BusOutput<SIZE> {
    fn from(value: u16) -> Self {
        array::from_fn(|bit| LineOutput::from(value >> bit & 1 == 1)).into()
    }
}

impl<const SIZE: usize> From<BitStringState<SIZE>> for BusOutput<SIZE> {
    fn from(value: BitStringState<SIZE>) -> Self {
        value.map(LineOutput::from).into()
    }
}

impl<T, const SIZE: usize> From<[T; SIZE]> for BusOutput<SIZE>
where
    T: Into<LineOutput>,
{
    fn from(value: [T; SIZE]) -> Self {
        value.map(Into::into).into()
    }
}

impl<const SIZE: usize> Combine for BusOutput<SIZE> {
    fn combine_with(&self, other: &Self) -> Self {
        array::from_fn(|bit| self[bit].combine_with(&other[bit])).into()
    }
}
