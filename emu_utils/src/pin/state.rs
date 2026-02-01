use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum PinState {
    #[strum(to_string = "high")]
    High,

    #[strum(to_string = "low")]
    Low,

    #[strum(to_string = "tri-stated")]
    TriState,

    #[strum(to_string = "undefined")]
    Undefined,
}

impl PinState {
    pub(crate) fn from_bool(b: bool) -> Self {
        if b { PinState::High } else { PinState::Low }
    }

    #[cfg(test)]
    pub(crate) fn as_bool(&self) -> Option<bool> {
        match self {
            PinState::High => Some(true),
            PinState::Low => Some(false),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(false, PinState::Low)]
    #[case(true, PinState::High)]
    fn from_bool(#[case] b: bool, #[case] state: PinState) {
        assert_eq!(PinState::from_bool(b), state);
    }

    #[rstest]
    #[case(PinState::Low, false)]
    #[case(PinState::High, true)]
    fn as_bool_success(#[case] state: PinState, #[case] b: bool) {
        assert_eq!(state.as_bool().unwrap(), b);
    }

    #[rstest]
    #[case(PinState::TriState)]
    #[case(PinState::Undefined)]
    fn as_bool_fail(#[case] state: PinState) {
        assert!(state.as_bool().is_none());
    }
}
