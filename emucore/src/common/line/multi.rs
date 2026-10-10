use crate::common::{
    combine::Combine,
    line::{error::LineError, ident::LineIdent, single::DriveState},
    read::{multi::MultiRead, single::SingleRead},
    signal::LineSignal,
};
use core::array;
use derive_more::{Deref, DerefMut, From, Index, IndexMut};

#[derive(Clone, Debug, Deref, DerefMut, Eq, From, Hash, Index, IndexMut, PartialEq)]
pub struct BusDriveState<const SIZE: usize>(pub [DriveState; SIZE]);

impl<const SIZE: usize> BusDriveState<SIZE> {
    #[must_use]
    pub fn from_multi_read(reads: &MultiRead<SIZE>) -> Self {
        reads.each_ref().map(|&read| read.into()).into()
    }

    #[must_use]
    pub fn from_signals(signals: &[LineSignal; SIZE]) -> Self {
        signals.each_ref().map(|&signal| signal.into()).into()
    }

    #[must_use]
    pub fn from_value(value: u16) -> Self {
        array::from_fn(|bit| DriveState::from(value >> bit & 1 == 1)).into()
    }

    pub fn read(&self) -> Result<MultiRead<SIZE>, usize> {
        let mut res: MultiRead<SIZE> = [SingleRead::Unknown; SIZE].into();

        for (bit, (read_bit, state)) in res.iter_mut().zip(self.iter()).enumerate() {
            *read_bit = state.read().ok_or(bit)?;
        }

        Ok(res)
    }

    pub fn read_ok(&self, name: &'static str) -> Result<MultiRead<SIZE>, LineError> {
        self.read().map_err(|bit| LineError::ImpossibleLineSignal {
            ident: LineIdent::BusLine {
                bus_name: name,
                bit,
            },
        })
    }

    pub fn contend(states: &[&Self]) -> Result<Self, usize> {
        let mut res: Self = [DriveState::none_enabled(); SIZE].into();

        for (bit, state) in res.iter_mut().enumerate() {
            *state = DriveState::contend(states.iter().map(|v| v[bit])).ok_or(bit)?;
        }

        Ok(res)
    }

    pub fn contend_ok(states: &[&Self], name: &'static str) -> Result<Self, LineError> {
        Self::contend(states).map_err(|bit| LineError::ShortCircuit {
            ident: LineIdent::BusLine {
                bus_name: name,
                bit,
            },
        })
    }
}

impl<const SIZE: usize> Combine for BusDriveState<SIZE> {
    fn combine_with(&self, other: &Self) -> Self {
        array::from_fn(|bit| self[bit].combine_with(&other[bit])).into()
    }
}
