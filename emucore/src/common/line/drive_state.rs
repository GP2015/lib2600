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
    pub const fn from_only(signal: LineSignal) -> Self {
        Self {
            low: matches!(signal, LineSignal::Low),
            high: matches!(signal, LineSignal::High),
            high_z: matches!(signal, LineSignal::HighZ),
        }
    }

    pub const fn from_drive(drive_val: SingleRead) -> Self {
        Self {
            low: matches!(drive_val, SingleRead::Low | SingleRead::Unknown),
            high: matches!(drive_val, SingleRead::High | SingleRead::Unknown),
            high_z: false,
        }
    }

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
