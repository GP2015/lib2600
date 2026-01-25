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
