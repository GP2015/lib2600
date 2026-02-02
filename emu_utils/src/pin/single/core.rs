use crate::pin::{PinError, PinState};

pub struct PinCore<E> {
    name: String,
    state: PinState,
    prev_state: PinState,
    err_type: std::marker::PhantomData<E>,
}

impl<E> PinCore<E> {
    pub fn new(name: String, initial_state: PinState) -> Self {
        Self {
            name,
            prev_state: PinState::TriState,
            state: initial_state,
            err_type: std::marker::PhantomData,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn state(&self) -> PinState {
        self.state
    }

    pub fn prev_state(&self) -> PinState {
        self.prev_state
    }

    pub fn state_as_bool(&self) -> Option<bool> {
        PinState::as_bool(&self.state)
    }

    pub fn prev_state_as_bool(&self) -> Option<bool> {
        PinState::as_bool(&self.prev_state)
    }

    pub fn set(&mut self, state: PinState) {
        self.prev_state = self.state;
        self.state = state;
    }
}

impl<E: From<PinError>> PinCore<E> {
    fn read_given_state(&self, state: PinState) -> Result<bool, E> {
        match state {
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

    pub fn read(&self) -> Result<bool, E> {
        self.read_given_state(self.state)
    }

    pub fn read_prev(&self) -> Result<bool, E> {
        self.read_given_state(self.prev_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = PinCore<PinError>;

    #[fixture]
    fn pin() -> PinType {
        PinCore::new(String::from("pin"), PinState::Undefined)
    }

    #[rstest]
    fn initial_state(
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        let pin = PinCore::<PinState>::new(String::new(), state);
        assert_eq!(pin.state(), state);
    }

    #[rstest]
    fn name(pin: PinType) {
        assert_eq!(pin.name(), String::from("pin"));
    }

    #[rstest]
    fn get_state(
        mut pin: PinType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        pin.set(state);
        assert_eq!(pin.state(), state);
    }

    #[rstest]
    #[case(PinState::High, Some(true))]
    #[case(PinState::Low, Some(false))]
    #[case(PinState::TriState, None)]
    #[case(PinState::Undefined, None)]
    fn state_as_bool(mut pin: PinType, #[case] state: PinState, #[case] b: Option<bool>) {
        pin.set(state);
        assert_eq!(pin.state_as_bool(), b);
    }

    #[rstest]
    fn read_bool(mut pin: PinType, #[values(true, false)] state: bool) {
        pin.set(PinState::from_bool(state));
        assert_eq!(pin.read().unwrap(), state);
    }

    #[rstest]
    fn read_tri_state(mut pin: PinType) {
        pin.set(PinState::TriState);
        assert!(matches!(
            pin.read().err().unwrap(),
            PinError::ReadTriStated { .. }
        ));
    }

    #[rstest]
    fn read_undefined(mut pin: PinType) {
        pin.set(PinState::Undefined);
        assert!(matches!(
            pin.read().err().unwrap(),
            PinError::ReadUndefined { .. }
        ));
    }
}
