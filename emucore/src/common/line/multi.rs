use crate::common::{
    line::{error::LineError, ident::LineIdent, single::DriveState},
    mux::{BaseCondition, HasMux, IsCondition},
    read::multi::MultiRead,
    signal::LineSignal,
};
use core::array;

pub type BusDriveState<const SIZE: usize> = [DriveState; SIZE];

pub trait IsBusDriveState<const SIZE: usize> {
    fn from_multi_read(reads: &MultiRead<SIZE>) -> Self;
    fn from_signals(signals: &[LineSignal; SIZE]) -> Self;
    fn from_usize(val: usize) -> Self;
    fn read(&self) -> MultiRead<SIZE>;
    #[must_use]
    fn combine_with(&self, other: &Self) -> Self;
    fn contend(name: &'static str, states: &[Self]) -> Result<Self, LineError>
    where
        Self: Sized;
}

impl<const SIZE: usize> IsBusDriveState<SIZE> for BusDriveState<SIZE> {
    fn from_multi_read(reads: &MultiRead<SIZE>) -> Self {
        array::from_fn(|bit| DriveState::from(reads[bit]))
    }

    fn from_signals(signals: &[LineSignal; SIZE]) -> Self {
        array::from_fn(|bit| DriveState::from(signals[bit]))
    }

    fn from_usize(value: usize) -> Self {
        array::from_fn(|bit| DriveState::from(value >> bit & 1 == 1))
    }

    fn read(&self) -> MultiRead<SIZE> {
        array::from_fn(|bit| self[bit].read())
    }

    fn combine_with(&self, other: &Self) -> Self {
        array::from_fn(|bit| self[bit].combine_with(other[bit]))
    }

    fn contend(bus_name: &'static str, states: &[Self]) -> Result<Self, LineError> {
        let mut res = [DriveState::default(); SIZE];

        for (bit, state) in res.iter_mut().enumerate() {
            let ident = LineIdent::BusLine { bus_name, bit };
            *state = DriveState::contend(ident, states.iter().map(|v| v[bit]))?;
        }

        Ok(res)
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
