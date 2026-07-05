use crate::common::{Combine, IsCondition, condition::BaseCondition};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SingleRead {
    Low,
    High,
    Unknown,
}

impl SingleRead {
    #[must_use]
    pub const fn as_bool(self) -> Option<bool> {
        match self {
            Self::High => Some(true),
            Self::Low => Some(false),
            Self::Unknown => None,
        }
    }

    #[must_use]
    pub const fn could_read_low(self) -> bool {
        matches!(self, Self::Low | Self::Unknown)
    }

    #[must_use]
    pub const fn could_read_high(self) -> bool {
        matches!(self, Self::High | Self::Unknown)
    }

    #[must_use]
    pub const fn could_read(self, state: bool) -> bool {
        if state {
            self.could_read_high()
        } else {
            self.could_read_low()
        }
    }

    #[must_use]
    pub const fn possible_reads(self) -> &'static [bool] {
        match self {
            Self::Low => &[false],
            Self::High => &[true],
            Self::Unknown => &[false, true],
        }
    }
}

impl From<bool> for SingleRead {
    fn from(value: bool) -> Self {
        if value { Self::High } else { Self::Low }
    }
}

impl IsCondition for SingleRead {
    fn as_cond(&self) -> BaseCondition {
        match self {
            Self::High => BaseCondition::Yes,
            Self::Low => BaseCondition::No,
            Self::Unknown => BaseCondition::Unknown,
        }
    }
}

impl Combine for SingleRead {
    fn combine_with(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Low, Self::Low) => Self::Low,
            (Self::High, Self::High) => Self::High,
            _ => Self::Unknown,
        }
    }
}
