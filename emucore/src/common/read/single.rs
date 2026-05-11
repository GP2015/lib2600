#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SingleRead {
    Low,
    High,
    Unknown,
}

impl SingleRead {
    pub fn could_read_low(self) -> bool {
        matches!(self, Self::Low | Self::Unknown)
    }

    pub fn could_read_high(self) -> bool {
        matches!(self, Self::High | Self::Unknown)
    }

    pub fn could_read(self, state: bool) -> bool {
        if state {
            self.could_read_high()
        } else {
            self.could_read_low()
        }
    }

    pub fn possible_reads(self) -> &'static [bool] {
        match self {
            Self::Low => &[false],
            Self::High => &[true],
            Self::Unknown => &[false, true],
        }
    }
}
