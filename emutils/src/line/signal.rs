use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum LineSignal {
    #[strum(to_string = "high")]
    High,

    #[strum(to_string = "low")]
    Low,

    #[strum(to_string = "high-impedance")]
    HighZ,
}

impl LineSignal {
    #[must_use]
    pub const fn from_bool(b: bool) -> Self {
        if b { Self::High } else { Self::Low }
    }

    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::High => Some(true),
            Self::Low => Some(false),
            Self::HighZ => None,
        }
    }

    #[must_use]
    pub const fn contend_with(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Self::Low, Self::Low) => Some(Self::Low),
            (Self::High, Self::High) => Some(Self::High),
            (any, Self::HighZ) | (Self::HighZ, any) => Some(any),
            (Self::Low, Self::High) | (Self::High, Self::Low) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(false, LineSignal::Low)]
    #[case(true, LineSignal::High)]
    fn from_bool(#[case] b: bool, #[case] signal: LineSignal) {
        assert_eq!(LineSignal::from_bool(b), signal);
    }

    #[rstest]
    #[case(LineSignal::Low, false)]
    #[case(LineSignal::High, true)]
    fn as_bool_success(#[case] signal: LineSignal, #[case] b: bool) {
        assert_eq!(signal.as_bool().unwrap(), b);
    }

    #[test]
    fn as_bool_failure() {
        assert!(LineSignal::HighZ.as_bool().is_none());
    }

    #[rstest]
    #[case(LineSignal::Low, LineSignal::Low, LineSignal::Low)]
    #[case(LineSignal::Low, LineSignal::HighZ, LineSignal::Low)]
    #[case(LineSignal::High, LineSignal::High, LineSignal::High)]
    #[case(LineSignal::High, LineSignal::HighZ, LineSignal::High)]
    #[case(LineSignal::HighZ, LineSignal::Low, LineSignal::Low)]
    #[case(LineSignal::HighZ, LineSignal::High, LineSignal::High)]
    #[case(LineSignal::HighZ, LineSignal::HighZ, LineSignal::HighZ)]
    fn contend_together_success(
        #[case] first: LineSignal,
        #[case] second: LineSignal,
        #[case] res: LineSignal,
    ) {
        assert_eq!(first.contend_with(second).unwrap(), res);
    }

    #[rstest]
    #[case(LineSignal::Low, LineSignal::High)]
    #[case(LineSignal::High, LineSignal::Low)]
    fn contend_together_failure(#[case] first: LineSignal, #[case] second: LineSignal) {
        assert!(first.contend_with(second).is_none());
    }
}
