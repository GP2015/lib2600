use crate::pin::{PinError, PinState};

pub type CallbackFn<O> = fn(&mut O, PinState, PinState) -> Result<(), PinError>;
