use crate::common::bit::single::BitState;
use core::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BaseCondition {
    No,
    Yes,
    #[default]
    Unknown,
}

impl BaseCondition {
    #[must_use]
    pub fn is_bool(self, b: bool) -> Self {
        if b { self } else { !self }
    }
}

impl From<bool> for BaseCondition {
    fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
    }
}

impl Not for BaseCondition {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::No => Self::Yes,
            Self::Yes => Self::No,
            Self::Unknown => Self::Unknown,
        }
    }
}

impl BitAnd for BaseCondition {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::No, _) | (_, Self::No) => Self::No,
            (Self::Yes, Self::Yes) => Self::Yes,
            _ => Self::Unknown,
        }
    }
}

impl BitOr for BaseCondition {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::No, Self::No) => Self::No,
            (Self::Yes, _) | (_, Self::Yes) => Self::Yes,
            _ => Self::Unknown,
        }
    }
}

impl From<BitState> for BaseCondition {
    fn from(value: BitState) -> Self {
        value.as_cond()
    }
}
