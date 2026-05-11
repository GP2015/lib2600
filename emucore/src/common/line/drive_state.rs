use crate::common::{read::single::SingleRead, signal::LineSignal};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DriveState {
    pub low: bool,
    pub high: bool,
    pub high_z: bool,
}

impl DriveState {
    pub fn from_only(signal: LineSignal) -> Self {
        Self {
            low: matches!(signal, LineSignal::Low),
            high: matches!(signal, LineSignal::High),
            high_z: matches!(signal, LineSignal::HighZ),
        }
    }

    pub fn is_valid(self) -> bool {
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
}
