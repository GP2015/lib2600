use crate::common::{combine::Combine, cond::BaseCondition, line::signal::LineSignal};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BitState {
    Low,
    High,
    #[default]
    Unknown,
}

impl BitState {
    #[must_use]
    pub const fn as_bool(self) -> Option<bool> {
        match self {
            Self::High => Some(true),
            Self::Low => Some(false),
            Self::Unknown => None,
        }
    }

    #[must_use]
    pub const fn as_cond(self) -> BaseCondition {
        match self {
            Self::High => BaseCondition::Yes,
            Self::Low => BaseCondition::No,
            Self::Unknown => BaseCondition::Unknown,
        }
    }

    #[must_use]
    pub const fn possible_values(self) -> &'static [bool] {
        match self {
            Self::Low => &[false],
            Self::High => &[true],
            Self::Unknown => &[false, true],
        }
    }
}

impl From<bool> for BitState {
    fn from(value: bool) -> Self {
        if value { Self::High } else { Self::Low }
    }
}

impl From<LineSignal> for BitState {
    fn from(value: LineSignal) -> Self {
        match value {
            LineSignal::Low => Self::Low,
            LineSignal::High => Self::High,
            LineSignal::HighZ => Self::Unknown,
        }
    }
}

impl Combine for BitState {
    fn combine_with(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Low, Self::Low) => Self::Low,
            (Self::High, Self::High) => Self::High,
            _ => Self::Unknown,
        }
    }
}
