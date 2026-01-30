use crate::{
    data::pins::{
        single::{SinglePin, SinglePinNew, SinglePinOutput},
        state::PinState,
    },
    error::RiotError,
};

pub struct ContentionPin {
    name: String,
    state: Option<PinState>,
    driving_in: bool,
    driving_out: bool,
}

impl ContentionPin {
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

impl SinglePinNew for ContentionPin {
    fn new(name: String) -> Self {
        Self {
            name,
            state: None,
            driving_in: false,
            driving_out: true,
        }
    }
}

impl SinglePin for ContentionPin {
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
        if matches!(state, PinState::TriState) {
            self.tristate_in();
            Ok(())
        } else {
            self.set_signal_in_bool_state(state)
        }
    }

    fn drive_in(&mut self, state: bool) -> Result<(), RiotError> {
        self.set_signal_in_bool_state(PinState::from_bool(state))
    }

    fn tristate_in(&mut self) {
        self.driving_in = false;
        if !self.driving_out {
            self.state = Some(PinState::TriState);
        }
    }
}

impl SinglePinOutput for ContentionPin {
    fn set_signal_out(&mut self, state: PinState) -> Result<(), RiotError> {
        if matches!(state, PinState::TriState) {
            self.tristate_out();
            Ok(())
        } else {
            self.set_signal_out_bool_state(state)
        }
    }

    fn drive_out(&mut self, state: bool) -> Result<(), RiotError> {
        self.set_signal_out_bool_state(PinState::from_bool(state))
    }

    fn tristate_out(&mut self) {
        self.driving_out = false;
        if !self.driving_in {
            self.state = Some(PinState::TriState);
        }
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
        reg.tristate_out();
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
    fn initial_bool_in(
        #[from(reg_default)] mut reg: ContentionPin,
        #[values(true, false)] state: bool,
    ) {
        assert!(matches!(
            reg.drive_in(state).err().unwrap(),
            RiotError::PotentialShortCircuit { .. }
        ));
    }

    #[rstest]
    fn initial_tristate_in(#[from(reg_default)] mut reg: ContentionPin) {
        reg.tristate_in();
        assert_eq!(reg.state(), None);
        assert!(matches!(
            reg.read().err().unwrap(),
            RiotError::PinUninitialised { .. }
        ));
    }

    #[rstest]
    fn set_and_state(
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
        reg.tristate_out();
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
    fn tristate_in(#[from(reg_tristate_out)] mut reg: ContentionPin) {
        reg.tristate_in();
        assert_eq!(reg.state().unwrap(), PinState::TriState);
    }

    #[rstest]
    fn tristate_out(#[from(reg_default)] mut reg: ContentionPin) {
        reg.tristate_out();
        assert_eq!(reg.state().unwrap(), PinState::TriState);
    }

    #[rstest]
    fn safe_contention(
        #[from(reg_default)] mut reg: ContentionPin,
        #[values(true, false)] state: bool,
    ) {
        reg.drive_out(state).unwrap();
        reg.drive_in(state).unwrap();
        assert_eq!(reg.read().unwrap(), state);
        reg.tristate_out();
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
