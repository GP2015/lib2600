use delegate::delegate;

use crate::pin::{
    PinError, PinState, SinglePin,
    single::{SinglePinNew, core::PinCore},
};

pub struct InputPin<E> {
    core: PinCore<E>,
}

impl<E> SinglePinNew for InputPin<E> {
    fn new(name: String) -> Self {
        Self {
            core: PinCore::new(name),
        }
    }
}

impl<E: From<PinError>> SinglePin for InputPin<E> {
    type Error = E;

    delegate! {
        to self.core{
            fn state(&self) -> PinState;
            fn state_as_bool(&self) -> Option<bool>;
            fn read(&self) -> Result<bool, Self::Error>;
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), E> {
        self.core.set(state);
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), E> {
        self.core.set(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_in(&mut self) {
        self.core.set(PinState::TriState);
    }

    fn undefine_in(&mut self) -> Result<(), E> {
        self.core.set(PinState::Undefined);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = InputPin<PinError>;

    #[fixture]
    fn reg() -> PinType {
        InputPin::new(String::new())
    }

    #[rstest]
    fn signal(
        mut reg: PinType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        reg.signal_in(state).unwrap();
        assert_eq!(reg.state(), state);
    }

    #[rstest]
    fn drive_in(mut reg: PinType, #[values(true, false)] b: bool) {
        reg.drive_in(b).unwrap();
        assert_eq!(reg.state(), PinState::from_bool(b));
    }

    #[rstest]
    fn tri_state_in(mut reg: PinType) {
        reg.tri_state_in();
        assert_eq!(reg.state(), PinState::TriState);
    }

    #[rstest]
    fn undefine_in(mut reg: PinType) {
        reg.undefine_in().unwrap();
        assert_eq!(reg.state(), PinState::Undefined);
    }
}
