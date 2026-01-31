use crate::pin::{PinError, PinState, SinglePin, single::SinglePinNew};

pub struct InputPin {
    name: String,
    state: Option<PinState>,
}

impl SinglePinNew for InputPin {
    fn new(name: String) -> Self {
        Self { name, state: None }
    }
}

impl SinglePin for InputPin {
    fn read(&self) -> Result<bool, PinError> {
        let Some(state) = self.state else {
            return Err(PinError::PinUninitialised {
                name: self.name.clone(),
            });
        };

        match state {
            PinState::High => Ok(true),
            PinState::Low => Ok(false),
            PinState::TriState => Err(PinError::PinReadWhileTriStated {
                name: self.name.clone(),
            }),
        }
    }

    fn state(&self) -> Option<PinState> {
        self.state
    }

    fn set_signal_in(&mut self, state: PinState) -> Result<(), PinError> {
        self.state = Some(state);
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), PinError> {
        self.state = Some(PinState::from_bool(state));
        Ok(())
    }

    fn tri_state_in(&mut self) {
        self.state = Some(PinState::TriState);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn reg() -> InputPin {
        InputPin::new(String::new())
    }

    #[rstest]
    fn initial_state(reg: InputPin) {
        assert_eq!(reg.state(), None);
        assert!(matches!(
            reg.read().err().unwrap(),
            PinError::PinUninitialised { .. }
        ));
    }

    #[rstest]
    fn set_and_state(
        mut reg: InputPin,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
    ) {
        reg.set_signal_in(state).unwrap();
        assert_eq!(reg.state().unwrap(), state);
    }

    #[rstest]
    #[case(true, PinState::High)]
    #[case(false, PinState::Low)]
    fn drive_in(mut reg: InputPin, #[case] istate: bool, #[case] ostate: PinState) {
        reg.drive_in(istate).unwrap();
        assert_eq!(reg.state().unwrap(), ostate);
    }

    #[rstest]
    fn tri_state_in(mut reg: InputPin) {
        reg.tri_state_in();
        assert_eq!(reg.state().unwrap(), PinState::TriState);
    }

    #[rstest]
    fn read_bool(mut reg: InputPin, #[values(true, false)] state: bool) {
        reg.drive_in(state).unwrap();
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_tri_state(mut reg: InputPin) {
        reg.tri_state_in();
        assert!(matches!(
            reg.read().err().unwrap(),
            PinError::PinReadWhileTriStated { .. }
        ));
    }
}
