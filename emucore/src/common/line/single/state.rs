use crate::common::{
    mux::{BaseCondition, HasMux, IsCondition},
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
    pub const fn is_valid(self) -> bool {
        self.low | self.high | self.high_z
    }

    pub fn read(self) -> SingleRead {
        match (self.low, self.high, self.high_z) {
            (false, false, false) => unreachable!(),
            (true, false, false) => SingleRead::Low,
            (false, true, false) => SingleRead::High,
            _ => SingleRead::Unknown,
        }
    }

    pub const fn combine_with(self, other: Self) -> Self {
        Self {
            low: self.low || other.low,
            high: self.high || other.high,
            high_z: self.high_z || other.high_z,
        }
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
