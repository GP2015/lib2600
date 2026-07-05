use crate::common::{
    Combine,
    line::{error::LineError, ident::LineIdent},
    read::single::SingleRead,
    signal::LineSignal,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DriveState {
    pub low: bool,
    pub high: bool,
    pub high_z: bool,
}

impl DriveState {
    #[must_use]
    pub const fn none_enabled() -> Self {
        Self {
            low: false,
            high: false,
            high_z: false,
        }
    }

    #[must_use]
    pub const fn read(self) -> Option<SingleRead> {
        match (self.low, self.high, self.high_z) {
            (false, false, false) => None,
            (true, false, false) => Some(SingleRead::Low),
            (false, true, false) => Some(SingleRead::High),
            _ => Some(SingleRead::Unknown),
        }
    }

    pub fn read_ok(self, ident: LineIdent) -> Result<SingleRead, LineError> {
        self.read().ok_or(LineError::ImpossibleLineSignal { ident })
    }

    fn contend_pair(self, other: Self) -> Option<Self> {
        let mut result = Self::none_enabled();

        let iter_possible = |state: Self| {
            [
                (state.low, LineSignal::Low),
                (state.high, LineSignal::High),
                (state.high_z, LineSignal::HighZ),
            ]
            .into_iter()
            .filter_map(|(enabled, signal)| enabled.then_some(signal))
        };

        for first_signal in iter_possible(self) {
            for second_signal in iter_possible(other) {
                match first_signal.contend_with(second_signal)? {
                    LineSignal::Low => result.low = true,
                    LineSignal::High => result.high = true,
                    LineSignal::HighZ => result.high_z = true,
                }
            }
        }

        Some(result)
    }

    pub fn contend(mut states: impl Iterator<Item = Self>) -> Option<Self> {
        let init = Self {
            low: false,
            high: false,
            high_z: true,
        };

        states.try_fold(init, Self::contend_pair)
    }

    pub fn contend_ok(
        states: impl Iterator<Item = Self>,
        ident: LineIdent,
    ) -> Result<Self, LineError> {
        Self::contend(states).ok_or(LineError::ShortCircuit { ident })
    }
}

impl From<SingleRead> for DriveState {
    fn from(value: SingleRead) -> Self {
        Self {
            low: matches!(value, SingleRead::Low | SingleRead::Unknown),
            high: matches!(value, SingleRead::High | SingleRead::Unknown),
            high_z: false,
        }
    }
}

impl From<LineSignal> for DriveState {
    fn from(value: LineSignal) -> Self {
        Self {
            low: matches!(value, LineSignal::Low),
            high: matches!(value, LineSignal::High),
            high_z: matches!(value, LineSignal::HighZ),
        }
    }
}

impl From<bool> for DriveState {
    fn from(value: bool) -> Self {
        Self {
            low: !value,
            high: value,
            high_z: false,
        }
    }
}

impl Combine for DriveState {
    fn combine_with(&self, other: &Self) -> Self {
        Self {
            low: self.low || other.low,
            high: self.high || other.high,
            high_z: self.high_z || other.high_z,
        }
    }
}
