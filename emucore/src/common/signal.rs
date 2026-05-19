use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum LineSignal {
    #[strum(to_string = "low")]
    Low,

    #[strum(to_string = "high")]
    High,

    #[strum(to_string = "high-impedance")]
    HighZ,
}

impl LineSignal {
    #[must_use]
    pub const fn as_bool(self) -> Option<bool> {
        match self {
            Self::Low => Some(false),
            Self::High => Some(true),
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

impl From<bool> for LineSignal {
    fn from(value: bool) -> Self {
        if value { Self::High } else { Self::Low }
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
        assert_eq!(LineSignal::from(b), signal);
    }

    #[rstest]
    #[case(LineSignal::Low, Some(false))]
    #[case(LineSignal::High, Some(true))]
    #[case(LineSignal::HighZ, None)]
    fn as_bool(#[case] signal: LineSignal, #[case] b: Option<bool>) {
        assert_eq!(signal.as_bool(), b);
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
