use crate::{
    RiotError,
    data::pins::{
        single::{SinglePin, SinglePinNew},
        state::PinState,
    },
};

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
    fn read(&self) -> Result<bool, RiotError> {
        let Some(state) = self.state else {
            return Err(RiotError::PinUninitialised {
                name: self.name.clone(),
            });
        };

        match state {
            PinState::High => Ok(true),
            PinState::Low => Ok(false),
            PinState::TriState => Err(RiotError::PinReadWhileTriStated {
                name: self.name.clone(),
            }),
        }
    }

    fn state(&self) -> Option<PinState> {
        self.state
    }

    fn set_signal_in(&mut self, state: PinState) -> Result<(), RiotError> {
        self.state = Some(state);
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), RiotError> {
        self.state = Some(PinState::from_bool(state));
        Ok(())
    }

    fn tristate_in(&mut self) {
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
            RiotError::PinUninitialised { .. }
        ));
    }

    #[rstest]
    fn set_and_state(
        mut reg: InputPin,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
    ) {
        reg.set_signal_in(state);
        assert_eq!(reg.state().unwrap(), state);
    }

    #[rstest]
    #[case(true, PinState::High)]
    #[case(false, PinState::Low)]
    fn drive_in(mut reg: InputPin, #[case] istate: bool, #[case] ostate: PinState) {
        reg.drive_in(istate);
        assert_eq!(reg.state().unwrap(), ostate);
    }

    #[rstest]
    fn tristate_in(mut reg: InputPin) {
        reg.tristate_in();
        assert_eq!(reg.state().unwrap(), PinState::TriState);
    }

    #[rstest]
    fn read_bool(mut reg: InputPin, #[values(true, false)] state: bool) {
        reg.drive_in(state);
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_tristate(mut reg: InputPin) {
        reg.tristate_in();
        assert!(matches!(
            reg.read().err().unwrap(),
            RiotError::PinReadWhileTriStated { .. }
        ));
    }
}
