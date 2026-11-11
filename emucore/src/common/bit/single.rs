use crate::common::{combine::Combine, cond::BaseCond, line::signal::LineSignal};

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
    pub const fn as_cond(self) -> BaseCond {
        match self {
            Self::High => BaseCond::Yes,
            Self::Low => BaseCond::No,
            Self::Unknown => BaseCond::Unknown,
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn default() {
        assert_eq!(BitState::default(), BitState::Unknown);
    }

    #[rstest]
    #[case(BitState::Low, Some(false))]
    #[case(BitState::High, Some(true))]
    #[case(BitState::Unknown, None)]
    fn as_bool(#[case] input: BitState, #[case] output: Option<bool>) {
        assert_eq!(input.as_bool(), output);
    }

    #[rstest]
    #[case(BitState::Low, BaseCond::No)]
    #[case(BitState::High, BaseCond::Yes)]
    #[case(BitState::Unknown, BaseCond::Unknown)]
    fn as_cond(#[case] input: BitState, #[case] output: BaseCond) {
        assert_eq!(input.as_cond(), output);
    }

    #[rstest]
    #[case(BitState::Low, &[false])]
    #[case(BitState::High, &[true])]
    #[case(BitState::Unknown, &[false, true])]
    fn possible_values(#[case] input: BitState, #[case] output: &[bool]) {
        assert_eq!(input.possible_values(), output);
    }

    #[rstest]
    #[case(false, BitState::Low)]
    #[case(true, BitState::High)]
    fn from_bool(#[case] b: bool, #[case] output: BitState) {
        assert_eq!(BitState::from(b), output);
    }

    #[rstest]
    #[case(LineSignal::Low, BitState::Low)]
    #[case(LineSignal::High, BitState::High)]
    #[case(LineSignal::HighZ, BitState::Unknown)]
    fn from_signal(#[case] input: LineSignal, #[case] output: BitState) {
        assert_eq!(BitState::from(input), output);
    }

    #[rstest]
    #[case(BitState::Low, BitState::Low, BitState::Low)]
    #[case(BitState::Low, BitState::High, BitState::Unknown)]
    #[case(BitState::Low, BitState::Unknown, BitState::Unknown)]
    #[case(BitState::High, BitState::Low, BitState::Unknown)]
    #[case(BitState::High, BitState::High, BitState::High)]
    #[case(BitState::High, BitState::Unknown, BitState::Unknown)]
    #[case(BitState::Unknown, BitState::Low, BitState::Unknown)]
    #[case(BitState::Unknown, BitState::High, BitState::Unknown)]
    #[case(BitState::Unknown, BitState::Unknown, BitState::Unknown)]
    fn combine_with(#[case] input0: BitState, #[case] input1: BitState, #[case] output: BitState) {
        assert_eq!(input0.combine_with(&input1), output);
    }
}
