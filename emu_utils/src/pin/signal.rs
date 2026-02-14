use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum PinSignal {
    #[strum(to_string = "high")]
    High,

    #[strum(to_string = "low")]
    Low,

    #[strum(to_string = "tri-signal")]
    TriState,
}

impl PinSignal {
    pub fn from_bool(b: bool) -> Self {
        if b { PinSignal::High } else { PinSignal::Low }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PinSignal::High => Some(true),
            PinSignal::Low => Some(false),
            _ => None,
        }
    }

    pub fn contend_together(first: Self, second: Self) -> Option<Self> {
        match (first, second) {
            (PinSignal::High, PinSignal::High) => Some(PinSignal::High),
            (PinSignal::Low, PinSignal::Low) => Some(PinSignal::Low),
            (any, PinSignal::TriState) => Some(any),
            (PinSignal::TriState, any) => Some(any),
            (PinSignal::High, PinSignal::Low) => None,
            (PinSignal::Low, PinSignal::High) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(false, PinSignal::Low)]
    #[case(true, PinSignal::High)]
    fn from_bool(#[case] b: bool, #[case] signal: PinSignal) {
        assert_eq!(PinSignal::from_bool(b), signal);
    }

    #[rstest]
    #[case(PinSignal::Low, false)]
    #[case(PinSignal::High, true)]
    fn as_bool_success(#[case] signal: PinSignal, #[case] b: bool) {
        assert_eq!(signal.as_bool().unwrap(), b);
    }

    #[rstest]
    fn as_bool_fail() {
        assert!(PinSignal::TriState.as_bool().is_none());
    }
}
