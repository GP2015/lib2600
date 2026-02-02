use crate::pin::{PinError, PinState};

pub struct PinCore<E> {
    name: String,
    state: PinState,
    err_type: std::marker::PhantomData<E>,
}

impl<E> PinCore<E> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: PinState::Undefined,
            err_type: std::marker::PhantomData,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn state(&self) -> PinState {
        self.state
    }

    pub fn state_as_bool(&self) -> Option<bool> {
        match self.state {
            PinState::High => Some(true),
            PinState::Low => Some(false),
            PinState::TriState => None,
            PinState::Undefined => None,
        }
    }

    pub fn set(&mut self, state: PinState) {
        self.state = state;
    }
}

impl<E: From<PinError>> PinCore<E> {
    pub fn read(&self) -> Result<bool, E> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = PinCore<PinError>;

    #[fixture]
    fn reg() -> PinType {
        PinCore::new(String::from("reg"))
    }

    #[rstest]
    fn initial_state(reg: PinType) {
        assert_eq!(reg.state(), PinState::Undefined);
    }

    #[rstest]
    fn name(reg: PinType) {
        assert_eq!(reg.name(), String::from("reg"));
    }

    #[rstest]
    fn get_state(
        mut reg: PinType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        reg.set(state);
        assert_eq!(reg.state(), state);
    }

    #[rstest]
    #[case(PinState::High, Some(true))]
    #[case(PinState::Low, Some(false))]
    #[case(PinState::TriState, None)]
    #[case(PinState::Undefined, None)]
    fn state_as_bool(mut reg: PinType, #[case] state: PinState, #[case] b: Option<bool>) {
        reg.set(state);
        assert_eq!(reg.state_as_bool(), b);
    }

    #[rstest]
    fn read_bool(mut reg: PinType, #[values(true, false)] state: bool) {
        reg.set(PinState::from_bool(state));
        assert_eq!(reg.read().unwrap(), state);
    }

    #[rstest]
    fn read_tri_state(mut reg: PinType) {
        reg.set(PinState::TriState);
        assert!(matches!(
            reg.read().err().unwrap(),
            PinError::ReadTriStated { .. }
        ));
    }

    #[rstest]
    fn read_undefined(mut reg: PinType) {
        reg.set(PinState::Undefined);
        assert!(matches!(
            reg.read().err().unwrap(),
            PinError::ReadUndefined { .. }
        ));
    }
}
