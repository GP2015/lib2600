use delegate::delegate;

use crate::pin::{PinError, PinState, SinglePinCore, SinglePinOutput, single::core::PinCore};

pub struct ContentionPin {
    core: PinCore,
    driving_in: bool,
    driving_out: bool,
}

#[derive(Clone, Copy)]
enum DriveDirection {
    In,
    Out,
}

impl ContentionPin {
    fn other_drive_direction_from(&self, dir: DriveDirection) -> bool {
        match dir {
            DriveDirection::In => self.driving_out,
            DriveDirection::Out => self.driving_in,
        }
    }

    fn set_this_drive_direction(&mut self, state: bool, dir: DriveDirection) {
        match dir {
            DriveDirection::In => self.driving_in = state,
            DriveDirection::Out => self.driving_out = state,
        }
    }

    fn signal(&mut self, state: PinState, drive_dir: DriveDirection) -> Result<(), PinError> {
        match state {
            PinState::High => self.drive(true, drive_dir)?,
            PinState::Low => self.drive(false, drive_dir)?,
            PinState::TriState => self.tri_state(drive_dir),
            PinState::Undefined => self.undefine(drive_dir)?,
        };
        Ok(())
    }

    fn drive(&mut self, next_state_b: bool, drive_dir: DriveDirection) -> Result<(), PinError> {
        let next_state = PinState::from_bool(next_state_b);

        if self.other_drive_direction_from(drive_dir) {
            if matches!(self.core.state(), PinState::Undefined) {
                return Err(PinError::PotentialShortCircuit {
                    name: self.core.name(),
                });
            };

            if self.core.state() != next_state {
                return Err(PinError::ShortCircuit {
                    name: self.core.name(),
                    current_state: self.core.state(),
                    next_state,
                });
            }
        }

        self.set_this_drive_direction(true, drive_dir);
        self.core.set_signal(next_state);
        Ok(())
    }

    fn tri_state(&mut self, drive_dir: DriveDirection) {
        self.set_this_drive_direction(false, drive_dir);
        if !self.other_drive_direction_from(drive_dir) {
            self.core.set_signal(PinState::TriState);
        }
    }

    fn undefine(&mut self, drive_dir: DriveDirection) -> Result<(), PinError> {
        if self.other_drive_direction_from(drive_dir) {
            return Err(PinError::PotentialShortCircuit {
                name: self.core.name(),
            });
        }

        self.set_this_drive_direction(true, drive_dir);
        self.core.set_signal(PinState::Undefined);
        Ok(())
    }
}

impl SinglePinCore for ContentionPin {
    fn new(name: String) -> Self {
        Self {
            core: PinCore::new(name, PinState::Undefined),
            driving_in: false,
            driving_out: true,
        }
    }

    delegate! {
        to self.core {
            fn state(&self) -> PinState;
            fn prev_state(&self) -> PinState;
            fn state_as_bool(&self) -> Option<bool>;
            fn prev_state_as_bool(&self) -> Option<bool>;
            fn read(&self) -> Result<bool, PinError>;
            fn read_prev(&self) -> Result<bool, PinError>;
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), PinError> {
        self.signal(state, DriveDirection::In)
    }

    fn drive_in(&mut self, state: bool) -> Result<(), PinError> {
        self.drive(state, DriveDirection::In)
    }

    fn tri_state_in(&mut self) {
        self.tri_state(DriveDirection::In)
    }

    fn undefine_in(&mut self) -> Result<(), PinError> {
        self.undefine(DriveDirection::In)
    }
}

impl SinglePinOutput for ContentionPin {
    fn signal_out(&mut self, state: PinState) -> Result<(), PinError> {
        self.signal(state, DriveDirection::Out)
    }

    fn drive_out(&mut self, state: bool) -> Result<(), PinError> {
        self.drive(state, DriveDirection::Out)
    }

    fn tri_state_out(&mut self) {
        self.tri_state(DriveDirection::Out)
    }

    fn undefine_out(&mut self) -> Result<(), PinError> {
        self.undefine(DriveDirection::Out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = ContentionPin;

    #[fixture]
    fn pin_default() -> PinType {
        ContentionPin::new(String::new())
    }

    #[rstest]
    fn initial_state(#[from(pin_default)] pin: PinType) {
        assert_eq!(pin.state(), PinState::Undefined);
    }

    #[fixture]
    fn pin_tri_state_out() -> PinType {
        let mut pin = ContentionPin::new(String::new());
        pin.tri_state_out();
        pin
    }

    type EmptyRes = Result<(), PinError>;
    type Signal = fn(&mut PinType, state: PinState) -> EmptyRes;
    type Drive = fn(&mut PinType, state: bool) -> EmptyRes;
    type TriState = fn(&mut PinType);
    type Undefine = fn(&mut PinType) -> EmptyRes;

    #[rstest]
    fn signal(
        #[from(pin_tri_state_out)] mut pin: PinType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
        #[values(ContentionPin::signal_in, ContentionPin::signal_out)] func: Signal,
    ) {
        func(&mut pin, state).unwrap();
        assert_eq!(pin.state(), state);
    }

    #[rstest]
    fn drive(
        #[from(pin_tri_state_out)] mut pin: PinType,
        #[values(true, false)] b: bool,
        #[values(ContentionPin::drive_in, ContentionPin::drive_out)] func: Drive,
    ) {
        func(&mut pin, b).unwrap();
        assert_eq!(pin.state(), PinState::from_bool(b));
    }

    #[rstest]
    fn tri_state(
        #[from(pin_tri_state_out)] mut pin: PinType,
        #[values(ContentionPin::tri_state_in, ContentionPin::tri_state_out)] func: TriState,
    ) {
        func(&mut pin);
        assert_eq!(pin.state(), PinState::TriState);
    }

    #[rstest]
    fn undefine(
        #[from(pin_tri_state_out)] mut pin: PinType,
        #[values(ContentionPin::undefine_in, ContentionPin::undefine_out)] func: Undefine,
    ) {
        func(&mut pin).unwrap();
        assert_eq!(pin.state(), PinState::Undefined);
    }

    #[rstest]
    #[case(PinState::TriState, PinState::TriState, PinState::TriState)]
    #[case(PinState::High, PinState::TriState, PinState::High)]
    #[case(PinState::Low, PinState::TriState, PinState::Low)]
    #[case(PinState::TriState, PinState::High, PinState::High)]
    #[case(PinState::TriState, PinState::Low, PinState::Low)]
    #[case(PinState::High, PinState::High, PinState::High)]
    #[case(PinState::Low, PinState::Low, PinState::Low)]
    fn safe_two_way_driving(
        #[from(pin_tri_state_out)] mut pin: PinType,
        #[case] istate: PinState,
        #[case] ostate: PinState,
        #[case] state: PinState,
    ) {
        pin.signal_in(istate).unwrap();
        pin.signal_out(ostate).unwrap();
        assert_eq!(pin.state(), state);
    }

    #[rstest]
    fn contention_swap(#[from(pin_default)] mut pin: PinType, #[values(true, false)] state: bool) {
        pin.drive_out(state).unwrap();
        pin.drive_in(state).unwrap();
        assert_eq!(pin.read().unwrap(), state);
        pin.tri_state_out();
        pin.drive_in(!state).unwrap();
        pin.drive_out(!state).unwrap();
        assert_eq!(pin.read().unwrap(), !state);
    }

    #[rstest]
    fn short_circuit(
        #[from(pin_tri_state_out)] mut pin: PinType,
        #[values(true, false)] state: bool,
        #[values(true, false)] dir: bool,
    ) {
        if dir {
            pin.drive_in(state).unwrap();
        } else {
            pin.drive_out(state).unwrap();
        }

        assert!(matches!(
            if dir {
                pin.drive_out(!state).err().unwrap()
            } else {
                pin.drive_in(!state).err().unwrap()
            },
            PinError::ShortCircuit { .. }
        ));
    }

    #[rstest]
    #[case(PinState::High, PinState::Undefined)]
    #[case(PinState::Low, PinState::Undefined)]
    #[case(PinState::Undefined, PinState::High)]
    #[case(PinState::Undefined, PinState::Low)]
    #[case(PinState::Undefined, PinState::Undefined)]
    fn potential_short_circuit(
        #[from(pin_tri_state_out)] mut pin: PinType,
        #[case] istate: PinState,
        #[case] ostate: PinState,
        #[values(true, false)] dir: bool,
    ) {
        if dir {
            pin.signal_in(istate).unwrap();
        } else {
            pin.signal_out(istate).unwrap();
        }

        assert!(matches!(
            if dir {
                pin.signal_out(ostate).err().unwrap()
            } else {
                pin.signal_in(ostate).err().unwrap()
            },
            PinError::PotentialShortCircuit { .. }
        ));
    }
}
