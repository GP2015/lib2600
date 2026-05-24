use crate::common::{
    HasMux, IsCondition,
    condition::BaseCondition,
    line::{error::LineError, ident::LineIdent, single::DriveState},
    read::{
        multi::{CheckMultiRead, MultiRead},
        single::SingleRead,
    },
    signal::LineSignal,
};
use core::array;

pub type BusDriveState<const SIZE: usize> = [DriveState; SIZE];

pub trait IsBusDriveState<const SIZE: usize> {
    fn from_multi_read(reads: &MultiRead<SIZE>) -> Self;
    fn from_signals(signals: &[LineSignal; SIZE]) -> Self;
    // fn from_value(val: u16) -> Self;
    fn read(&self) -> Result<MultiRead<SIZE>, usize>;
    // fn read_or_error(&self, name: &'static str) -> Result<MultiRead<SIZE>, LineError>;
    #[must_use]
    fn combine_with(&self, other: &Self) -> Self;
    fn contend(states: &[Self]) -> Result<Self, usize>
    where
        Self: Sized;
}

impl<const SIZE: usize> IsBusDriveState<SIZE> for BusDriveState<SIZE> {
    fn from_multi_read(reads: &MultiRead<SIZE>) -> Self {
        reads.each_ref().map(|&read| read.into())
    }

    fn from_signals(signals: &[LineSignal; SIZE]) -> Self {
        signals.each_ref().map(|&signal| signal.into())
    }

    // fn from_value(value: u16) -> Self {
    //     array::from_fn(|bit| DriveState::from(value >> bit & 1 == 1))
    // }

    fn read(&self) -> Result<MultiRead<SIZE>, usize> {
        let mut res = [SingleRead::Unknown; SIZE];

        for (bit, (read_bit, state)) in res.iter_mut().zip(self.iter()).enumerate() {
            *read_bit = state.read().ok_or(bit)?;
        }

        Ok(res)
    }

    // fn read_or_error(&self, name: &'static str) -> Result<MultiRead<SIZE>, LineError> {
    //     self.read().ok_or_impossible(name)
    // }

    fn combine_with(&self, other: &Self) -> Self {
        array::from_fn(|bit| self[bit].combine_with(other[bit]))
    }

    fn contend(states: &[Self]) -> Result<Self, usize> {
        let mut res = [DriveState::none_enabled(); SIZE];

        for (bit, state) in res.iter_mut().enumerate() {
            *state = DriveState::contend(states.iter().map(|v| v[bit])).ok_or(bit)?;
        }

        Ok(res)
    }
}

impl<const SIZE: usize> HasMux for BusDriveState<SIZE> {
    fn mux(cond: BaseCondition, low_opt: &impl Fn() -> Self, high_opt: &impl Fn() -> Self) -> Self {
        match cond.as_cond() {
            BaseCondition::No => low_opt(),
            BaseCondition::Yes => high_opt(),
            BaseCondition::Unknown => low_opt().combine_with(&high_opt()),
        }
    }
}

pub trait CheckBusDriveState<const SIZE: usize> {
    fn ok_or_short(self, name: &'static str) -> Result<BusDriveState<SIZE>, LineError>;
    fn ok_read_or_error(self, name: &'static str) -> Result<MultiRead<SIZE>, LineError>;
}

impl<const SIZE: usize> CheckBusDriveState<SIZE> for Result<BusDriveState<SIZE>, usize> {
    fn ok_or_short(self, name: &'static str) -> Result<BusDriveState<SIZE>, LineError> {
        match self {
            Ok(state) => Ok(state),
            Err(bit) => Err(LineError::ShortCircuit {
                ident: LineIdent::BusLine {
                    bus_name: name,
                    bit,
                },
            }),
        }
    }

    fn ok_read_or_error(self, name: &'static str) -> Result<MultiRead<SIZE>, LineError> {
        self.ok_or_short(name)?.read().ok_or_impossible(name)
    }
}
