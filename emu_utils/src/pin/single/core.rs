use crate::pin::{PinError, PinSignal, PinState};
use std::marker::PhantomData;

pub struct PinCore<E> {
    name: String,
    state: PinState,
    prev_state: PinState,
    err_type: PhantomData<E>,
}

impl<E> PinCore<E> {
    pub fn new(name: String, initial_state: PinState) -> Self {
        Self {
            name,
            prev_state: PinState::new(high, low, tri_state),
            state: initial_state,
            err_type: PhantomData,
        }
    }

    pub fn set_signal(&mut self, state: PinSignal) {
        self.state = state;
    }

    pub fn post_tick_update(&mut self) {
        self.prev_state = self.state;
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn state(&self) -> PinSignal {
        self.state
    }

    pub fn prev_state(&self) -> PinSignal {
        self.prev_state
    }

    pub fn state_as_bool(&self) -> Option<bool> {
        PinSignal::as_bool(&self.state)
    }

    pub fn prev_state_as_bool(&self) -> Option<bool> {
        PinSignal::as_bool(&self.prev_state)
    }
}

impl<E: From<PinError>> PinCore<E> {
    fn read_given_state(&self, state: PinSignal) -> Result<bool, E> {
        match state {
            PinSignal::High => Ok(true),
            PinSignal::Low => Ok(false),
            PinSignal::TriState => Err(E::from(PinError::ReadTriStated {
                name: self.name.clone(),
            })),
            PinSignal::Undefined => Err(E::from(PinError::ReadUndefined {
                name: self.name.clone(),
            })),
        }
    }

    pub fn read(&self) -> Result<bool, E> {
        self.read_given_state(self.state)
    }

    pub fn read_prev(&self) -> Result<bool, E> {
        self.read_given_state(self.prev_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    type PinType = PinCore<PinError>;

    #[fixture]
    fn pin() -> PinType {
        let mut pin = PinCore::new(String::from("pin"), PinSignal::Undefined);
        pin.post_tick_update();
        pin
    }

    #[rstest]
    fn initial_state(
        #[values(PinSignal::High, PinSignal::Low, PinSignal::TriState)] state: PinSignal,
    ) {
        let pin = PinCore::<PinError>::new(String::new(), state);
        assert_eq!(pin.state(), state);
        assert_eq!(pin.prev_state(), PinSignal::TriState);
    }

    #[rstest]
    fn post_tick_update(
        mut pin: PinCore<PinError>,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::TriState)] state: PinSignal,
    ) {
        pin.set_signal(state);
        assert_eq!(pin.state(), state);
        assert_eq!(pin.prev_state(), PinSignal::Undefined);
        pin.post_tick_update();
        assert_eq!(pin.state(), state);
        assert_eq!(pin.prev_state(), state);
    }

    #[rstest]
    fn name(pin: PinType) {
        assert_eq!(pin.name(), String::from("pin"));
    }

    #[rstest]
    fn get_state_and_prev(
        mut pin: PinType,
        #[values(
            PinSignal::High,
            PinSignal::Low,
            PinSignal::TriState,
            PinSignal::Undefined
        )]
        state: PinSignal,
    ) {
        pin.set_signal(state);
        assert_eq!(pin.state(), state);
        assert_eq!(pin.prev_state(), PinSignal::Undefined);
        pin.post_tick_update();
        pin.set_signal(state);
        assert_eq!(pin.prev_state(), state);
    }

    #[rstest]
    #[case(PinSignal::High, Some(true))]
    #[case(PinSignal::Low, Some(false))]
    #[case(PinSignal::TriState, None)]
    #[case(PinSignal::Undefined, None)]
    fn state_as_bool(mut pin: PinType, #[case] state: PinSignal, #[case] b: Option<bool>) {
        pin.set_signal(state);
        assert_eq!(pin.state_as_bool(), b);
        assert_eq!(pin.prev_state_as_bool(), None);
        pin.post_tick_update();
        pin.set_signal(state);
        assert_eq!(pin.prev_state_as_bool(), b);
    }

    #[rstest]
    fn read_bool(mut pin: PinType, #[values(true, false)] state: bool) {
        pin.set_signal(PinSignal::from_bool(state));
        pin.post_tick_update();
        pin.set_signal(PinSignal::from_bool(!state));
        assert_eq!(pin.read_prev().unwrap(), state);
        assert_eq!(pin.read().unwrap(), !state);
    }

    fn expect_tri_stated_err(res: Result<bool, PinError>) {
        assert!(matches!(res.err().unwrap(), PinError::ReadTriStated { .. }));
    }

    #[rstest]
    fn read_tri_state(
        mut pin: PinType,
        #[values(PinSignal::High, PinSignal::Low, PinSignal::Undefined)] state: PinSignal,
    ) {
        pin.set_signal(PinSignal::TriState);
        expect_tri_stated_err(pin.read());
        pin.post_tick_update();
        pin.set_signal(state);
        expect_tri_stated_err(pin.read_prev());
    }
}
