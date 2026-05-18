use crate::common::{
    BaseCondition, HasMux, IsCondition,
    line::{error::LineError, ident::LineIdent},
    read::single::SingleRead,
    signal::LineSignal,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DriveState {
    pub low: bool,
    pub high: bool,
    pub high_z: bool,
}

impl DriveState {
    #[must_use]
    pub fn read(self) -> SingleRead {
        match (self.low, self.high, self.high_z) {
            (false, false, false) => unreachable!(),
            (true, false, false) => SingleRead::Low,
            (false, true, false) => SingleRead::High,
            _ => SingleRead::Unknown,
        }
    }

    #[must_use]
    pub const fn combine_with(self, other: Self) -> Self {
        Self {
            low: self.low || other.low,
            high: self.high || other.high,
            high_z: self.high_z || other.high_z,
        }
    }

    fn contend_pair(self, other: Self) -> Option<Self> {
        let mut result = Self::default();

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

    pub fn contend(
        ident: impl Into<LineIdent>,
        mut states: impl Iterator<Item = Self>,
    ) -> Result<Self, LineError> {
        let init = Self {
            low: false,
            high: false,
            high_z: true,
        };

        states
            .try_fold(init, Self::contend_pair)
            .ok_or_else(|| LineError::ShortCircuit {
                ident: ident.into(),
            })
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

impl IsCondition for DriveState {
    fn as_cond(&self) -> BaseCondition {
        match (self.low, self.high, self.high_z) {
            (false, false, false) => unreachable!(),
            (true, false, false) => BaseCondition::No,
            (false, true, false) => BaseCondition::Yes,
            _ => BaseCondition::Unknown,
        }
    }
}

impl HasMux for DriveState {
    fn mux(
        cond: &impl IsCondition,
        low_opt: &impl Fn() -> Self,
        high_opt: &impl Fn() -> Self,
    ) -> Self {
        match cond.as_cond() {
            BaseCondition::No => low_opt(),
            BaseCondition::Yes => high_opt(),
            BaseCondition::Unknown => low_opt().combine_with(high_opt()),
        }
    }
}
