use crate::common::{
    combine::Combine,
    line::single::DriveState,
    read::{multi::MultiRead, single::SingleRead},
};
use core::array;
use derive_more::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, Index, IndexMut, PartialEq)]
pub struct BusDriveState<const SIZE: usize>([DriveState; SIZE]);

impl<const SIZE: usize> BusDriveState<SIZE> {
    #[must_use]
    pub fn read(&self) -> Option<MultiRead<SIZE>> {
        let mut read: MultiRead<SIZE> = [SingleRead::Unknown; SIZE].into();

        for (read_bit, state) in read.iter_mut().zip(self.iter()) {
            *read_bit = state.read()?;
        }

        Some(read)
    }

    #[must_use]
    pub fn contend(states: &[&Self]) -> Option<Self> {
        let mut res: Self = [DriveState::default(); SIZE].into();

        for (bit, state) in res.iter_mut().enumerate() {
            *state = DriveState::contend(states.iter().map(|v| v[bit]))?;
        }

        Some(res)
    }
}

impl<const SIZE: usize> From<u16> for BusDriveState<SIZE> {
    fn from(value: u16) -> Self {
        array::from_fn(|bit| DriveState::from(value >> bit & 1 == 1)).into()
    }
}

impl<const SIZE: usize> From<&MultiRead<SIZE>> for BusDriveState<SIZE> {
    fn from(value: &MultiRead<SIZE>) -> Self {
        value.map(DriveState::from).into()
    }
}

impl<T: Into<DriveState>, const SIZE: usize> From<[T; SIZE]> for BusDriveState<SIZE> {
    fn from(value: [T; SIZE]) -> Self {
        value.map(Into::into).into()
    }
}

impl<const SIZE: usize> Combine for BusDriveState<SIZE> {
    fn combine_with(&self, other: &Self) -> Self {
        array::from_fn(|bit| self[bit].combine_with(&other[bit])).into()
    }
}
