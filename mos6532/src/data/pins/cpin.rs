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

// Add unit tests here
