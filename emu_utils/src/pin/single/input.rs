use crate::pin::{
    PinError, PinState, SinglePinInput,
    single::{CallbackFn, SinglePinSetup, core::PinCore},
};
use delegate::delegate;

pub struct InputPin<O> {
    core: PinCore<O>,
}

impl<O> SinglePinSetup<O> for InputPin<O> {
    fn new(name: String) -> Self {
        Self {
            core: PinCore::new(name, PinState::TriState),
        }
    }

    delegate! {
        to self.core {
            fn assign_callback(&mut self, callback: Box<CallbackFn<O>>);
        }
    }
}

impl<O> SinglePinInput for InputPin<O> {
    delegate! {
        to self.core {
            fn state(&self) -> PinState;
            fn state_as_bool(&self) -> Option<bool>;
            fn read(&self) -> Result<bool, PinError>;
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), PinError> {
        self.core.set_signal(state)
    }

    fn drive_in(&mut self, state: bool) -> Result<(), PinError> {
        self.core.set_signal(PinState::from_bool(state))
    }

    fn tri_state_in(&mut self) -> Result<(), PinError> {
        self.core.set_signal(PinState::TriState)
    }

    fn undefine_in(&mut self) -> Result<(), PinError> {
        self.core.set_signal(PinState::Undefined)
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
        pin.tri_state_in().unwrap();
        assert_eq!(pin.state(), PinState::TriState);
    }

    #[rstest]
    fn undefine_in(mut pin: PinType) {
        pin.undefine_in().unwrap();
        assert_eq!(pin.state(), PinState::Undefined);
    }
}
