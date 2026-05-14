use std::array;

use crate::common::{
    line::single::state::DriveState,
    mux::{BaseCondition, HasMux, IsCondition},
    read::{multi::MultiRead, single::SingleRead},
    signal::LineSignal,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BusDriveState<const SIZE: usize> {
    inner: [DriveState; SIZE],
}

impl<const SIZE: usize> BusDriveState<SIZE> {
    pub const fn bit<const BIT: usize>(&self) -> DriveState {
        const { assert!(BIT < SIZE) }
        self.inner[BIT]
    }

    pub fn try_bit(&self, bit: usize) -> Option<DriveState> {
        self.inner.get(bit).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = DriveState> {
        self.inner.iter().copied()
    }
}

impl<const SIZE: usize> BusDriveState<SIZE> {
    pub fn combine_with(&self, other: &Self) -> Self {
        Self {
            inner: array::from_fn(|bit| self.inner[bit].combine_with(other.inner[bit])),
        }
    }
}

impl<const SIZE: usize> From<[DriveState; SIZE]> for BusDriveState<SIZE> {
    fn from(value: [DriveState; SIZE]) -> Self {
        Self { inner: value }
    }
}

impl<const SIZE: usize> From<&MultiRead<SIZE>> for BusDriveState<SIZE> {
    fn from(value: &MultiRead<SIZE>) -> Self {
        Self {
            inner: array::from_fn(|bit| DriveState::from(value.try_bit(bit).unwrap())),
        }
    }
}

impl<const SIZE: usize> From<[SingleRead; SIZE]> for BusDriveState<SIZE> {
    fn from(value: [SingleRead; SIZE]) -> Self {
        Self {
            inner: array::from_fn(|bit| DriveState::from(value[bit])),
        }
    }
}

impl<const SIZE: usize> From<[LineSignal; SIZE]> for BusDriveState<SIZE> {
    fn from(value: [LineSignal; SIZE]) -> Self {
        Self {
            inner: array::from_fn(|bit| DriveState::from(value[bit])),
        }
    }
}

impl<const SIZE: usize> From<usize> for BusDriveState<SIZE> {
    fn from(value: usize) -> Self {
        Self {
            inner: array::from_fn(|bit| DriveState::from(value >> bit & 1 == 1)),
        }
    }
}

impl<const SIZE: usize> HasMux for BusDriveState<SIZE> {
    fn mux(
        cond: &impl IsCondition,
        low_opt: &impl Fn() -> Self,
        high_opt: &impl Fn() -> Self,
    ) -> Self {
        match cond.as_cond() {
            BaseCondition::No => low_opt(),
            BaseCondition::Yes => high_opt(),
            BaseCondition::Unknown => low_opt().combine_with(&high_opt()),
        }
    }
}
