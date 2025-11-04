pub struct State {
    active: bool,
}

impl State {
    pub fn new() -> Self {
        Self { active: true }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_active() {
        let mut state = State::new();
        assert_eq!(state.is_active(), true);
        state.deactivate();
        assert_eq!(state.is_active(), false);
    }
}
