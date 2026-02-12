use delegate::delegate;

use crate::pin::{PinError, PinState, SinglePinCore, SinglePinInterface, single::core::PinCore};

pub struct InputPin<E> {
    core: PinCore<E>,
}

impl<E: From<PinError>> SinglePinInterface<E> for InputPin<E> {
    delegate! {
        to self.core {
            fn state(&self) -> PinState;
            fn prev_state(&self) -> PinState;
            fn state_as_bool(&self) -> Option<bool>;
            fn prev_state_as_bool(&self) -> Option<bool>;
            fn read(&self) -> Result<bool, E>;
            fn read_prev(&self) -> Result<bool, E>;
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), E> {
        self.core.set_signal(state);
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), E> {
        self.core.set_signal(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_in(&mut self) {
        self.core.set_signal(PinState::TriState);
    }

    fn undefine_in(&mut self) -> Result<(), E> {
        self.core.set_signal(PinState::Undefined);
        Ok(())
    }
}

impl<E> SinglePinCore for InputPin<E> {
    fn new(name: String) -> Self {
        Self {
            core: PinCore::new(name, PinState::TriState),
        }
    }

    delegate! {
        to self.core {
            fn post_tick_update(&mut self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = InputPin<PinError>;

    #[fixture]
    fn pin() -> PinType {
        InputPin::new(String::new())
    }

    #[rstest]
    fn initial_state(pin: PinType) {
        assert_eq!(pin.state(), PinState::TriState);
    }

    #[rstest]
    fn signal(
        mut pin: PinType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        pin.signal_in(state).unwrap();
        assert_eq!(pin.state(), state);
    }

    #[rstest]
    fn drive_in(mut pin: PinType, #[values(true, false)] b: bool) {
        pin.drive_in(b).unwrap();
        assert_eq!(pin.state(), PinState::from_bool(b));
    }

    #[rstest]
    fn tri_state_in(mut pin: PinType) {
        pin.tri_state_in();
        assert_eq!(pin.state(), PinState::TriState);
    }

    #[rstest]
    fn undefine_in(mut pin: PinType) {
        pin.undefine_in().unwrap();
        assert_eq!(pin.state(), PinState::Undefined);
    }
}
