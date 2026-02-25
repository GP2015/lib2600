use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum PinSignal {
    #[strum(to_string = "high")]
    High,

    #[strum(to_string = "low")]
    Low,

    #[strum(to_string = "high-impedance")]
    HighZ,
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
            (PinSignal::Low, PinSignal::Low) => Some(PinSignal::Low),
            (PinSignal::High, PinSignal::High) => Some(PinSignal::High),
            (any, PinSignal::HighZ) => Some(any),
            (PinSignal::HighZ, any) => Some(any),
            (PinSignal::Low, PinSignal::High) => None,
            (PinSignal::High, PinSignal::Low) => None,
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
    fn as_bool_failure() {
        assert!(PinSignal::HighZ.as_bool().is_none());
    }

    #[rstest]
    #[case(PinSignal::Low, PinSignal::Low, PinSignal::Low)]
    #[case(PinSignal::Low, PinSignal::HighZ, PinSignal::Low)]
    #[case(PinSignal::High, PinSignal::High, PinSignal::High)]
    #[case(PinSignal::High, PinSignal::HighZ, PinSignal::High)]
    #[case(PinSignal::HighZ, PinSignal::Low, PinSignal::Low)]
    #[case(PinSignal::HighZ, PinSignal::High, PinSignal::High)]
    #[case(PinSignal::HighZ, PinSignal::HighZ, PinSignal::HighZ)]
    fn contend_together_success(
        #[case] first: PinSignal,
        #[case] second: PinSignal,
        #[case] result: PinSignal,
    ) {
        let o: PinSignal = PinSignal::contend_together(first, second).unwrap();
        assert_eq!(o, result);
    }

    #[rstest]
    #[case(PinSignal::Low, PinSignal::High)]
    #[case(PinSignal::High, PinSignal::Low)]
    fn contend_together_failure(#[case] first: PinSignal, #[case] second: PinSignal) {
        assert!(PinSignal::contend_together(first, second).is_none());
    }
}
