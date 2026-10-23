use crate::common::{bit::single::BitState, combine::Combine, line::signal::LineSignal};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct LineOutput {
    pub low: bool,
    pub high: bool,
    pub high_z: bool,
}

impl LineOutput {
    #[must_use]
    pub const fn new(low: bool, high: bool, high_z: bool) -> Self {
        Self { low, high, high_z }
    }

    #[must_use]
    pub const fn read(self) -> Option<BitState> {
        match (self.low, self.high, self.high_z) {
            (false, false, false) => None,
            (true, false, false) => Some(BitState::Low),
            (false, true, false) => Some(BitState::High),
            _ => Some(BitState::Unknown),
        }
    }

    #[must_use]
    pub fn contend_with(self, other: Self) -> Option<Self> {
        let mut result = Self::new(false, false, false);

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
}

impl From<LineSignal> for LineOutput {
    fn from(value: LineSignal) -> Self {
        Self {
            low: matches!(value, LineSignal::Low),
            high: matches!(value, LineSignal::High),
            high_z: matches!(value, LineSignal::HighZ),
        }
    }
}

impl From<BitState> for LineOutput {
    fn from(value: BitState) -> Self {
        Self {
            low: matches!(value, BitState::Low | BitState::Unknown),
            high: matches!(value, BitState::High | BitState::Unknown),
            high_z: false,
        }
    }
}

impl From<bool> for LineOutput {
    fn from(value: bool) -> Self {
        Self {
            low: !value,
            high: value,
            high_z: false,
        }
    }
}

impl Combine for LineOutput {
    fn combine_with(&self, other: &Self) -> Self {
        Self {
            low: self.low || other.low,
            high: self.high || other.high,
            high_z: self.high_z || other.high_z,
        }
    }
}
