use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum PinState {
    #[strum(to_string = "high")]
    High,

    #[strum(to_string = "low")]
    Low,

    #[strum(to_string = "tri-stated")]
    TriState,
}

impl PinState {
    pub(crate) fn from_bool(b: bool) -> Self {
        if b { PinState::High } else { PinState::Low }
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
}
