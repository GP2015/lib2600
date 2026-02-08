use crate::pin::{PinError, PinState, single::CallbackFn};

pub struct PinCore<O> {
    name: String,
    callback: Option<Box<CallbackFn<O>>>,
    state: PinState,
    prev_state: PinState,
}

impl<O> PinCore<O> {
    pub fn new(name: String, initial_state: PinState) -> Self {
        Self {
            name,
            callback: None,
            prev_state: PinState::TriState,
            state: initial_state,
        }
    }

    pub fn assign_callback(&mut self, callback: Box<CallbackFn<O>>) {
        self.callback = Some(callback);
    }

    fn handle_callback(&mut self) -> Result<(), PinError> {
        if self.prev_state != self.state
            && let Some(callback) = self.callback.as_mut()
        {
            callback(self.prev_state, self.state)
        } else {
            Ok(())
        }
    }

    pub fn set_signal(&mut self, state: PinState) -> Result<(), PinError> {
        self.prev_state = self.state;
        self.state = state;
        self.handle_callback()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn state(&self) -> PinState {
        self.state
    }

    pub fn state_as_bool(&self) -> Option<bool> {
        PinState::as_bool(&self.state)
    }

    pub fn read(&self) -> Result<bool, PinError> {
        match self.state {
            PinState::High => Ok(true),
            PinState::Low => Ok(false),
            PinState::TriState => Err(PinError::ReadTriStated {
                name: self.name.clone(),
            }),
            PinState::Undefined => Err(PinError::ReadUndefined {
                name: self.name.clone(),
            }),
        }
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
        let pin = PinCore::<PinError>::new(String::new(), state);
        assert_eq!(pin.state(), state);
    }

    #[rstest]
    fn name(pin: PinType) {
        assert_eq!(pin.name(), String::from("pin"));
    }

    #[rstest]
    fn get_state_and_prev(
        mut pin: PinType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        pin.set_signal(state).unwrap();
        assert_eq!(pin.state(), state);
    }

    #[rstest]
    #[case(PinState::High, Some(true))]
    #[case(PinState::Low, Some(false))]
    #[case(PinState::TriState, None)]
    #[case(PinState::Undefined, None)]
    fn state_as_bool(mut pin: PinType, #[case] state: PinState, #[case] b: Option<bool>) {
        pin.set_signal(state).unwrap();
        assert_eq!(pin.state_as_bool(), b);
    }

    #[rstest]
    fn read_bool(mut pin: PinType, #[values(true, false)] state: bool) {
        pin.set_signal(PinState::from_bool(state)).unwrap();
        assert_eq!(pin.read().unwrap(), state);
    }

    fn expect_tri_stated_err(res: Result<bool, PinError>) {
        assert!(matches!(res.err().unwrap(), PinError::ReadTriStated { .. }));
    }

    #[rstest]
    fn read_tri_state(mut pin: PinType) {
        pin.set_signal(PinState::TriState).unwrap();
        expect_tri_stated_err(pin.read());
    }

    fn expect_undefined_err(res: Result<bool, PinError>) {
        assert!(matches!(res.err().unwrap(), PinError::ReadUndefined { .. }));
    }

    #[rstest]
    fn read_undefined(mut pin: PinType) {
        pin.set_signal(PinState::Undefined).unwrap();
        expect_undefined_err(pin.read());
    }
}
