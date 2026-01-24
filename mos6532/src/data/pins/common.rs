#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PinState {
    High,
    Low,
    TriState,
    Undefined,
}
