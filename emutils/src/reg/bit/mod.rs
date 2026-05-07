pub mod state;

use crate::{line::LineState, reg::bit::state::BitRegisterState};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BitRegister {
    name: String,
    state: BitRegisterState,
}

impl BitRegister {
    #[must_use]
    pub fn new<S: Into<String>>(name: S, low: bool, high: bool) -> Self {
        Self {
            name: name.into(),
            state: BitRegisterState::new(low, high),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn state(&self) -> BitRegisterState {
        self.state
    }

    pub const fn add(&mut self, state: bool) {
        if state {
            self.state.high = true;
        } else {
            self.state.low = true;
        }
    }

    pub const fn set_all(&mut self, high: bool, low: bool) {
        self.state.high = high;
        self.state.low = low;
    }

    pub const fn remove(&mut self, state: bool) {
        if state {
            self.state.high = false;
        } else {
            self.state.low = false;
        }
    }

    pub const fn remove_all(&mut self) {
        self.set_all(false, false);
    }

    pub const fn copy_from_line_state(&mut self, line: &LineState) {
        if line.could_read_high() {
            self.state.high = true;
        }

        if line.could_read_low() {
            self.state.low = true;
        }
    }

    pub const fn copy_from_reg_state(&mut self, reg: &BitRegisterState) {
        if reg.high {
            self.state.high = true;
        }

        if reg.low {
            self.state.low = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::line::{Line, LineConnectionId};

    use super::*;
    use rstest::{fixture, rstest};

    const REG_NAME: &str = "reg";

    #[fixture]
    fn line_and_connection() -> (Line, LineConnectionId) {
        let mut line = Line::new("");
        let connection = line.create_connection();
        (line, connection)
    }

    #[fixture]
    fn reg() -> BitRegister {
        BitRegister::new(REG_NAME, true, true)
    }

    #[rstest]
    fn initial(reg: BitRegister) {
        assert!(reg.state().low);
        assert!(reg.state().high);
    }

    #[rstest]
    fn name(reg: BitRegister) {
        assert_eq!(reg.name(), REG_NAME);
    }

    #[rstest]
    fn copy_from_line_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high: bool,
        mut reg: BitRegister,
        #[from(line_and_connection)] (mut line, connection): (Line, LineConnectionId),
    ) {
        reg.set_all(initial, initial);
        line.set_all(connection, high, low, false).unwrap();
        reg.copy_from_line_state(&line.state());
        assert_eq!(reg.state().high, high);
        assert_eq!(reg.state().low, low);
    }

    #[rstest]
    fn copy_from_line_not_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high: bool,
        mut reg: BitRegister,
        #[from(line_and_connection)] (mut line, connection): (Line, LineConnectionId),
    ) {
        reg.set_all(initial, initial);
        line.set_all(connection, high, low, false).unwrap();
        reg.copy_from_line_state(&line.state());
        assert_eq!(reg.state().high, high | initial);
        assert_eq!(reg.state().low, low | initial);
    }
}
