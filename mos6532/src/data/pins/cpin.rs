use crate::{data::pins::common::PinState, error::RiotError};

const DRIVING_IN: usize = 0;
const DRIVING_OUT: usize = 1;

pub enum DriveDirection {
    In,
    Out,
}

pub struct ContentionPin {
    state: PinState,
    driving: [bool; 2],
}

impl ContentionPin {
    pub fn new() -> Self {
        let mut driving = [false; 2];
        driving[DRIVING_OUT] = true;

        Self {
            state: PinState::Undefined,
            driving,
        }
    }

    pub fn read(&self) -> PinState {
        self.state
    }

    pub fn set_drive(&mut self, dir: DriveDirection, state: PinState) -> Result<(), RiotError> {
        let (this, other) = match dir {
            DriveDirection::In => (DRIVING_IN, DRIVING_OUT),
            DriveDirection::Out => (DRIVING_OUT, DRIVING_IN),
        };

        if matches!(state, PinState::TriState) {
            self.driving[this] = false;
            if !self.driving[other] {
                self.state = PinState::TriState;
            }
            return Ok(());
        }

        if self.driving[other] && (matches!(state, PinState::Undefined) || state != self.state) {
            return Err(RiotError::ShortCircuit);
        }
        self.driving[this] = true;
        self.state = state;
        Ok(())
    }

    pub fn set_drive_in(&mut self, state: PinState) -> Result<(), RiotError> {
        self.set_drive(DriveDirection::In, state)
    }

    pub fn set_drive_out(&mut self, state: PinState) -> Result<(), RiotError> {
        self.set_drive(DriveDirection::Out, state)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::{fixture, rstest};

//     #[fixture]
//     fn reg() -> OutputPin {
//         OutputPin::new()
//     }

//     #[rstest]
//     fn get_initial(reg: OutputPin) {
//         assert_eq!(reg.get(), PinState::Undefined);
//     }

//     #[rstest]
//     #[case(PinState::High)]
//     fn set_get(mut reg: OutputPin, #[case] state: PinState) {
//         reg.set(state);
//         assert_eq!(reg.get(), state);
//     }
// }
