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

impl From<BitState> for BaseCond {
    fn from(value: BitState) -> Self {
        value.as_cond()
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn default() {
        assert_eq!(BaseCond::default(), BaseCond::Unknown);
    }

    #[rstest]
    #[case(BaseCond::No, false, BaseCond::Yes)]
    #[case(BaseCond::Unknown, false, BaseCond::Unknown)]
    #[case(BaseCond::Yes, false, BaseCond::No)]
    #[case(BaseCond::No, true, BaseCond::No)]
    #[case(BaseCond::Unknown, true, BaseCond::Unknown)]
    #[case(BaseCond::Yes, true, BaseCond::Yes)]
    fn is_bool(#[case] input: BaseCond, #[case] b: bool, #[case] output: BaseCond) {
        assert_eq!(input.is_bool(b), output);
    }

    #[rstest]
    #[case(false, BaseCond::No)]
    #[case(true, BaseCond::Yes)]
    fn from_bool(#[case] b: bool, #[case] output: BaseCond) {
        assert_eq!(BaseCond::from(b), output);
    }

    #[rstest]
    #[case(BaseCond::No, BaseCond::Yes)]
    #[case(BaseCond::Yes, BaseCond::No)]
    #[case(BaseCond::Unknown, BaseCond::Unknown)]
    fn not(#[case] input: BaseCond, #[case] output: BaseCond) {
        assert_eq!(!input, output);
    }

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
