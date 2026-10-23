use crate::common::bit::single::BitState;
use core::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BaseCond {
    No,
    Yes,
    #[default]
    Unknown,
}

impl BaseCond {
    #[must_use]
    pub fn is_bool(self, b: bool) -> Self {
        if b { self } else { !self }
    }
}

impl From<bool> for BaseCond {
    fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
    }
}

impl Not for BaseCond {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::No => Self::Yes,
            Self::Yes => Self::No,
            Self::Unknown => Self::Unknown,
        }
    }
}

impl BitAnd for BaseCond {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::No, _) | (_, Self::No) => Self::No,
            (Self::Yes, Self::Yes) => Self::Yes,
            _ => Self::Unknown,
        }
    }
}

impl BitOr for BaseCond {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::No, Self::No) => Self::No,
            (Self::Yes, _) | (_, Self::Yes) => Self::Yes,
            _ => Self::Unknown,
        }
    }
}

impl BitXor for BaseCond {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::No, Self::No) | (Self::Yes, Self::Yes) => Self::No,
            (Self::No, Self::Yes) | (Self::Yes, Self::No) => Self::Yes,
            _ => Self::Unknown,
        }
    }
}

impl From<BitState> for BaseCond {
    fn from(value: BitState) -> Self {
        value.as_cond()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(BaseCond::No, BaseCond::No, BaseCond::No)]
    #[case(BaseCond::No, BaseCond::Yes, BaseCond::No)]
    #[case(BaseCond::No, BaseCond::Unknown, BaseCond::No)]
    #[case(BaseCond::Yes, BaseCond::Yes, BaseCond::Yes)]
    #[case(BaseCond::Yes, BaseCond::Unknown, BaseCond::Unknown)]
    #[case(BaseCond::Unknown, BaseCond::Unknown, BaseCond::Unknown)]
    fn and(#[case] input0: BaseCond, #[case] input1: BaseCond, #[case] output: BaseCond) {
        assert_eq!(input0 & input1, output);
    }

    #[rstest]
    #[case(BaseCond::No, BaseCond::No, BaseCond::No)]
    #[case(BaseCond::No, BaseCond::Yes, BaseCond::Yes)]
    #[case(BaseCond::No, BaseCond::Unknown, BaseCond::Unknown)]
    #[case(BaseCond::Yes, BaseCond::Yes, BaseCond::Yes)]
    #[case(BaseCond::Yes, BaseCond::Unknown, BaseCond::Yes)]
    #[case(BaseCond::Unknown, BaseCond::Unknown, BaseCond::Unknown)]
    fn or(#[case] input0: BaseCond, #[case] input1: BaseCond, #[case] output: BaseCond) {
        assert_eq!(input0 | input1, output);
    }

    #[rstest]
    #[case(BaseCond::No, BaseCond::No, BaseCond::No)]
    #[case(BaseCond::No, BaseCond::Yes, BaseCond::Yes)]
    #[case(BaseCond::No, BaseCond::Unknown, BaseCond::Unknown)]
    #[case(BaseCond::Yes, BaseCond::Yes, BaseCond::No)]
    #[case(BaseCond::Yes, BaseCond::Unknown, BaseCond::Unknown)]
    #[case(BaseCond::Unknown, BaseCond::Unknown, BaseCond::Unknown)]
    fn xor(#[case] input0: BaseCond, #[case] input1: BaseCond, #[case] output: BaseCond) {
        assert_eq!(input0 ^ input1, output);
    }
}
