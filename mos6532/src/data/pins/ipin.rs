use crate::{RiotError, data::pins::common::PinState};

pub struct InputPin {
    name: String,
    state: Option<PinState>,
}

impl InputPin {
    pub(crate) fn new(name: String) -> Self {
        Self { name, state: None }
    }

    pub fn read(&self) -> Result<bool, RiotError> {
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

    pub fn state(&self) -> Option<PinState> {
        self.state
    }

    pub fn set_signal_in(&mut self, state: PinState) {
        self.state = Some(state);
    }

    pub fn drive_in(&mut self, state: bool) {
        self.state = Some(PinState::from_bool(state));
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
    fn read_bool(mut reg: InputPin, #[values(true, false)] state: bool) {
        reg.drive_in(state);
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_tristate(mut reg: InputPin) {
        reg.set_signal_in(PinState::TriState);
        assert!(matches!(
            reg.read().err().unwrap(),
            RiotError::PinReadWhileTriStated { .. }
        ));
    }
}
