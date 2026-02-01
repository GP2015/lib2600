use crate::pin::{PinError, PinState, SinglePin, SinglePinOutput, single::SinglePinNew};

pub struct ContentionPin<E> {
    name: String,
    state: PinState,
    driving_in: bool,
    driving_out: bool,
    err_type: std::marker::PhantomData<E>,
}

#[derive(Clone, Copy)]
enum DriveDirection {
    In,
    Out,
}

impl<E: From<PinError>> ContentionPin<E> {
    fn other_drive_direction_from(&self, dir: DriveDirection) -> bool {
        match dir {
            DriveDirection::In => self.driving_out,
            DriveDirection::Out => self.driving_in,
        }
    }

    fn set_this_drive_direction(&mut self, dir: DriveDirection, state: bool) {
        match dir {
            DriveDirection::In => self.driving_in = state,
            DriveDirection::Out => self.driving_out = state,
        }
    }

    fn drive(&mut self, next_state_b: bool, drive_dir: DriveDirection) -> Result<(), E> {
        let next_state = PinState::from_bool(next_state_b);

        if self.other_drive_direction_from(drive_dir) {
            if matches!(self.state, PinState::Undefined) {
                return Err(E::from(PinError::PotentialShortCircuit {
                    name: self.name.clone(),
                }));
            };

            if self.state != next_state {
                return Err(E::from(PinError::ShortCircuit {
                    name: self.name.clone(),
                    current_state: self.state,
                    next_state,
                }));
            }
        }

        self.set_this_drive_direction(drive_dir, true);
        self.state = next_state;
        Ok(())
    }

    fn tri_state(&mut self, drive_dir: DriveDirection) {
        self.set_this_drive_direction(drive_dir, false);
        if !self.other_drive_direction_from(drive_dir) {
            self.state = PinState::TriState;
        }
    }

    fn undefine(&mut self, drive_dir: DriveDirection) -> Result<(), E> {
        if self.other_drive_direction_from(drive_dir) {
            return Err(E::from(PinError::PotentialShortCircuit {
                name: self.name.clone(),
            }));
        }

        self.set_this_drive_direction(drive_dir, true);
        self.state = PinState::Undefined;
        Ok(())
    }
}

impl<E> SinglePinNew for ContentionPin<E> {
    fn new(name: String) -> Self {
        Self {
            name,
            state: PinState::Undefined,
            driving_in: false,
            driving_out: true,
            err_type: std::marker::PhantomData,
        }
    }
}

impl<E: From<PinError>> SinglePin for ContentionPin<E> {
    type Error = E;

    fn state(&self) -> PinState {
        self.state
    }

    fn read(&self) -> Result<bool, E> {
        match self.state {
            PinState::High => Ok(true),
            PinState::Low => Ok(false),
            PinState::TriState => Err(E::from(PinError::ReadTriStated {
                name: self.name.clone(),
            })),
            PinState::Undefined => Err(E::from(PinError::ReadUndefined {
                name: self.name.clone(),
            })),
        }
    }

    fn signal_in(&mut self, state: PinState) -> Result<(), E> {
        match state {
            PinState::High => self.drive(true, DriveDirection::In)?,
            PinState::Low => self.drive(false, DriveDirection::In)?,
            PinState::TriState => self.tri_state(DriveDirection::In),
            PinState::Undefined => self.undefine(DriveDirection::In)?,
        };
        Ok(())
    }

    fn drive_in(&mut self, state: bool) -> Result<(), E> {
        self.drive(state, DriveDirection::In)
    }

    fn tri_state_in(&mut self) {
        self.tri_state(DriveDirection::In)
    }

    fn undefine_in(&mut self) -> Result<(), E> {
        self.undefine(DriveDirection::In)
    }
}

impl<E: From<PinError>> SinglePinOutput for ContentionPin<E> {
    type Error = E;

    fn signal_out(&mut self, state: PinState) -> Result<(), E> {
        match state {
            PinState::High => self.drive(true, DriveDirection::Out)?,
            PinState::Low => self.drive(false, DriveDirection::Out)?,
            PinState::TriState => self.tri_state(DriveDirection::Out),
            PinState::Undefined => self.undefine(DriveDirection::Out)?,
        };
        Ok(())
    }

    fn drive_out(&mut self, state: bool) -> Result<(), E> {
        self.drive(state, DriveDirection::Out)
    }

    fn tri_state_out(&mut self) {
        self.tri_state(DriveDirection::Out)
    }

    fn undefine_out(&mut self) -> Result<(), E> {
        self.undefine(DriveDirection::Out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = ContentionPin<PinError>;

    #[fixture]
    fn reg_default() -> PinType {
        ContentionPin::new(String::new())
    }

    #[fixture]
    fn reg_tri_state_out() -> PinType {
        let mut reg = ContentionPin::new(String::new());
        reg.tri_state_out();
        reg
    }

    type EmptyRes = Result<(), PinError>;
    type Signal = fn(&mut PinType, state: PinState) -> EmptyRes;
    type Drive = fn(&mut PinType, state: bool) -> EmptyRes;
    type TriState = fn(&mut PinType);
    type Undefine = fn(&mut PinType) -> EmptyRes;

    #[rstest]
    fn initial_state(#[from(reg_default)] reg: PinType) {
        assert_eq!(reg.state(), PinState::Undefined);
    }

    #[rstest]
    fn get_state(
        #[from(reg_default)] mut reg: PinType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        reg.signal_out(state).unwrap();
        assert_eq!(reg.state(), state);
    }

    #[rstest]
    fn read_bool(#[from(reg_default)] mut reg: PinType, #[values(true, false)] state: bool) {
        reg.drive_out(state).unwrap();
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_tri_state(#[from(reg_default)] mut reg: PinType) {
        reg.tri_state_out();
        assert!(matches!(
            reg.read().err().unwrap(),
            PinError::ReadTriStated { .. }
        ));
    }

    #[rstest]
    fn read_undefined(#[from(reg_default)] mut reg: PinType) {
        reg.undefine_out().unwrap();
        assert!(matches!(
            reg.read().err().unwrap(),
            PinError::ReadUndefined { .. }
        ));
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
        #[from(reg_tri_state_out)] mut reg: PinType,
        #[case] istate: PinState,
        #[case] ostate: PinState,
        #[case] state: PinState,
    ) {
        reg.signal_in(istate).unwrap();
        reg.signal_out(ostate).unwrap();
        assert_eq!(reg.state(), state);
    }

    #[rstest]
    fn drive(
        #[from(reg_tri_state_out)] mut reg: PinType,
        #[values(true, false)] b: bool,
        #[values(ContentionPin::drive_in, ContentionPin::drive_out)] func: Drive,
    ) {
        func(&mut reg, b).unwrap();
        assert_eq!(reg.state(), PinState::from_bool(b));
    }

    #[rstest]
    fn signal(
        #[from(reg_tri_state_out)] mut reg: PinType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
        #[values(ContentionPin::signal_in, ContentionPin::signal_out)] func: Signal,
    ) {
        func(&mut reg, state).unwrap();
        assert_eq!(reg.state(), state);
    }

    #[rstest]
    fn tri_state(
        #[from(reg_tri_state_out)] mut reg: PinType,
        #[values(ContentionPin::tri_state_in, ContentionPin::tri_state_out)] func: TriState,
    ) {
        func(&mut reg);
        assert_eq!(reg.state(), PinState::TriState);
    }

    #[rstest]
    fn undefine(
        #[from(reg_tri_state_out)] mut reg: PinType,
        #[values(ContentionPin::undefine_in, ContentionPin::undefine_out)] func: Undefine,
    ) {
        func(&mut reg).unwrap();
        assert_eq!(reg.state(), PinState::Undefined);
    }

    #[rstest]
    fn contention_swap(#[from(reg_default)] mut reg: PinType, #[values(true, false)] state: bool) {
        reg.drive_out(state).unwrap();
        reg.drive_in(state).unwrap();
        assert_eq!(reg.read().unwrap(), state);
        reg.tri_state_out();
        reg.drive_in(!state).unwrap();
        reg.drive_out(!state).unwrap();
        assert_eq!(reg.read().unwrap(), !state);
    }

    #[rstest]
    fn short_circuit(
        #[from(reg_tri_state_out)] mut reg: PinType,
        #[values(true, false)] state: bool,
        #[values(true, false)] dir: bool,
    ) {
        if dir {
            reg.drive_in(state).unwrap();
        } else {
            reg.drive_out(state).unwrap();
        }

        assert!(matches!(
            if dir {
                reg.drive_out(!state).err().unwrap()
            } else {
                reg.drive_in(!state).err().unwrap()
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
        #[from(reg_tri_state_out)] mut reg: PinType,
        #[case] istate: PinState,
        #[case] ostate: PinState,
        #[values(true, false)] dir: bool,
    ) {
        if dir {
            reg.signal_in(istate).unwrap();
        } else {
            reg.signal_out(istate).unwrap();
        }

        assert!(matches!(
            if dir {
                reg.signal_out(ostate).err().unwrap()
            } else {
                reg.signal_in(ostate).err().unwrap()
            },
            PinError::PotentialShortCircuit { .. }
        ));
    }
}
