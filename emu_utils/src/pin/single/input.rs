use crate::pin::{PinError, PinState, SinglePin, single::SinglePinNew};

pub struct InputPin<E> {
    name: String,
    state: PinState,
    err_type: std::marker::PhantomData<E>,
}

impl<E> SinglePinNew for InputPin<E> {
    fn new(name: String) -> Self {
        Self {
            name,
            state: PinState::Undefined,
            err_type: std::marker::PhantomData,
        }
    }
}

impl<E: From<PinError>> SinglePin for InputPin<E> {
    type Error = E;

    fn state(&self) -> PinState {
        self.state
    }

    fn read(&self) -> Result<bool, E> {
        match self.state {
            PinState::High => Ok(true),
            PinState::Low => Ok(false),
            PinState::TriState => Err(E::from(PinError::ReadTriStated {
                name: self.name.clone(),
            })),
            PinState::Undefined => Err(E::from(PinError::ReadUndefined {
                name: self.name.clone(),
            })),
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), E> {
        self.state = state;
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), E> {
        self.state = PinState::from_bool(state);
        Ok(())
    }

    fn tri_state_in(&mut self) {
        self.state = PinState::TriState;
    }

    fn undefine_in(&mut self) -> Result<(), E> {
        self.state = PinState::Undefined;
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
    fn initial_state(reg: PinType) {
        assert_eq!(reg.state(), PinState::Undefined);
    }

    #[rstest]
    fn set_and_state(
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

    #[rstest]
    fn read_bool(mut reg: PinType, #[values(true, false)] state: bool) {
        reg.drive_in(state).unwrap();
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_tri_state(mut reg: PinType) {
        reg.tri_state_in();
        assert!(matches!(
            reg.read().err().unwrap(),
            PinError::ReadTriStated { .. }
        ));
    }

    #[rstest]
    fn read_undefined(mut reg: PinType) {
        reg.undefine_in().unwrap();
        assert!(matches!(
            reg.read().err().unwrap(),
            PinError::ReadUndefined { .. }
        ));
    }
}
