use crate::{data::pins::common::PinState, error::RiotError};

pub struct ContentionPin {
    name: String,
    state: Option<PinState>,
    driving_in: bool,
    driving_out: bool,
}

impl ContentionPin {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            state: None,
            driving_in: false,
            driving_out: true,
        }
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

    pub fn drive_in(&mut self, state: bool) -> Result<(), RiotError> {
        self.set_signal_in_bool_state(PinState::from_bool(state))
    }

    pub(crate) fn drive_out(&mut self, state: bool) -> Result<(), RiotError> {
        self.set_signal_out_bool_state(PinState::from_bool(state))
    }

    pub fn set_signal_in(&mut self, state: PinState) -> Result<(), RiotError> {
        if matches!(state, PinState::TriState) {
            self.driving_in = false;
            if !self.driving_out {
                self.state = Some(PinState::TriState);
            }
            return Ok(());
        }
        self.set_signal_in_bool_state(state)
    }

    pub(crate) fn set_signal_out(&mut self, state: PinState) -> Result<(), RiotError> {
        if matches!(state, PinState::TriState) {
            self.driving_out = false;
            if !self.driving_in {
                self.state = Some(PinState::TriState);
            }
            return Ok(());
        }
        self.set_signal_out_bool_state(state)
    }

    fn set_signal_in_bool_state(&mut self, state: PinState) -> Result<(), RiotError> {
        if self.driving_out {
            let Some(current_state) = self.state else {
                return Err(RiotError::PotentialShortCircuit {
                    name: self.name.clone(),
                    state,
                });
            };

            if current_state != state {
                return Err(RiotError::ShortCircuit {
                    name: self.name.clone(),
                    current_state,
                    next_state: state,
                });
            }
        }

        self.driving_in = true;
        self.state = Some(state);
        Ok(())
    }

    fn set_signal_out_bool_state(&mut self, state: PinState) -> Result<(), RiotError> {
        if self.driving_in {
            let current_state = self.state.unwrap();

            if current_state != state {
                return Err(RiotError::ShortCircuit {
                    name: self.name.clone(),
                    current_state,
                    next_state: state,
                });
            }
        }

        self.driving_out = true;
        self.state = Some(state);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn reg_default() -> ContentionPin {
        ContentionPin::new(String::new())
    }

    #[fixture]
    fn reg_tristate_out() -> ContentionPin {
        let mut reg = ContentionPin::new(String::new());
        reg.set_signal_out(PinState::TriState).unwrap();
        reg
    }

    #[rstest]
    fn initial_state(#[from(reg_default)] reg: ContentionPin) {
        assert_eq!(reg.state(), None);
        assert!(matches!(
            reg.read().err().unwrap(),
            RiotError::PinUninitialised { .. }
        ));
    }

    #[rstest]
    fn initial_set_in_bool(
        #[from(reg_default)] mut reg: ContentionPin,
        #[values(PinState::High, PinState::Low)] state: PinState,
    ) {
        assert!(matches!(
            reg.set_signal_in(state).err().unwrap(),
            RiotError::PotentialShortCircuit { .. }
        ));
    }

    #[rstest]
    fn initial_set_in_tristate(#[from(reg_default)] mut reg: ContentionPin) {
        reg.set_signal_in(PinState::TriState).unwrap();
        assert_eq!(reg.state(), None);
        assert!(matches!(
            reg.read().err().unwrap(),
            RiotError::PinUninitialised { .. }
        ));
    }

    #[rstest]
    fn get_state(
        #[from(reg_default)] mut reg: ContentionPin,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
    ) {
        reg.set_signal_out(state).unwrap();
        assert_eq!(reg.state().unwrap(), state);
    }

    #[rstest]
    fn read_bool(#[from(reg_default)] mut reg: ContentionPin, #[values(true, false)] state: bool) {
        reg.drive_out(state).unwrap();
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_tristate(#[from(reg_default)] mut reg: ContentionPin) {
        reg.set_signal_out(PinState::TriState).unwrap();
        assert!(matches!(
            reg.read().err().unwrap(),
            RiotError::PinReadWhileTriStated { .. }
        ));
    }

    #[rstest]
    #[case(PinState::TriState, PinState::TriState, PinState::TriState)]
    #[case(PinState::High, PinState::TriState, PinState::High)]
    #[case(PinState::Low, PinState::TriState, PinState::Low)]
    #[case(PinState::TriState, PinState::High, PinState::High)]
    #[case(PinState::TriState, PinState::Low, PinState::Low)]
    fn safe_bool_reads(
        #[from(reg_tristate_out)] mut reg: ContentionPin,
        #[case] istate: PinState,
        #[case] ostate: PinState,
        #[case] state: PinState,
    ) {
        reg.set_signal_in(istate).unwrap();
        reg.set_signal_out(ostate).unwrap();
        assert_eq!(reg.state().unwrap(), state);
    }

    #[rstest]
    #[case(true, PinState::High)]
    #[case(false, PinState::Low)]
    fn drive_in(
        #[from(reg_tristate_out)] mut reg: ContentionPin,
        #[case] istate: bool,
        #[case] ostate: PinState,
    ) {
        reg.drive_out(istate).unwrap();
        assert_eq!(reg.state().unwrap(), ostate);
    }

    #[rstest]
    #[case(true, PinState::High)]
    #[case(false, PinState::Low)]
    fn drive_out(
        #[from(reg_default)] mut reg: ContentionPin,
        #[case] istate: bool,
        #[case] ostate: PinState,
    ) {
        reg.drive_out(istate).unwrap();
        assert_eq!(reg.state().unwrap(), ostate);
    }

    #[rstest]
    fn safe_contention(
        #[from(reg_default)] mut reg: ContentionPin,
        #[values(true, false)] state: bool,
    ) {
        reg.drive_out(state).unwrap();
        reg.drive_in(state).unwrap();
        assert_eq!(reg.read().unwrap(), state);
        reg.set_signal_out(PinState::TriState).unwrap();
        reg.drive_in(!state).unwrap();
        reg.drive_out(!state).unwrap();
        assert_eq!(reg.read().unwrap(), !state);
    }

    #[rstest]
    fn in_short_circuit(
        #[from(reg_default)] mut reg: ContentionPin,
        #[values(true, false)] state: bool,
    ) {
        reg.drive_out(state).unwrap();
        assert!(matches!(
            reg.drive_in(!state).err().unwrap(),
            RiotError::ShortCircuit { .. }
        ));
    }

    #[rstest]
    fn out_short_circuit(
        #[from(reg_tristate_out)] mut reg: ContentionPin,
        #[values(true, false)] state: bool,
    ) {
        reg.drive_in(state).unwrap();
        assert!(matches!(
            reg.drive_out(!state).err().unwrap(),
            RiotError::ShortCircuit { .. }
        ));
    }
}
