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
    fn read_initial(reg: InputPin) {
        assert!(reg.read().is_err());
    }

    #[rstest]
    fn drive_read(mut reg: InputPin) {
        reg.set_signal_in(PinState::High);
        assert!(reg.read().unwrap());
    }
}
