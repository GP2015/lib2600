pub mod state;

use crate::{line::LineState, reg::bit::state::BitRegState};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BitReg {
    name: String,
    state: BitRegState,
}

impl BitReg {
    #[must_use]
    pub fn new<S: Into<String>>(name: S, low: bool, high: bool) -> Self {
        Self {
            name: name.into(),
            state: BitRegState::new(low, high),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn state(&self) -> BitRegState {
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

    pub const fn copy_from_reg_state(&mut self, reg: &BitRegState) {
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
    use crate::line::{Line, LineConId};

    use super::*;
    use rstest::{fixture, rstest};

    const REG_NAME: &str = "reg";

    #[fixture]
    fn line_and_connection() -> (Line, LineConId) {
        let mut line = Line::new("");
        let connection = line.create_connection();
        (line, connection)
    }

    #[fixture]
    fn reg() -> BitReg {
        BitReg::new(REG_NAME, true, true)
    }

    #[rstest]
    fn initial(reg: BitReg) {
        assert!(reg.state().low);
        assert!(reg.state().high);
    }

    #[rstest]
    fn name(reg: BitReg) {
        assert_eq!(reg.name(), REG_NAME);
    }

    #[rstest]
    fn copy_from_line_only_possible(
        #[values(true, false)] initial: bool,
        #[values(true, false)] low: bool,
        #[values(true, false)] high: bool,
        mut reg: BitReg,
        #[from(line_and_connection)] (mut line, connection): (Line, LineConId),
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
        mut reg: BitReg,
        #[from(line_and_connection)] (mut line, connection): (Line, LineConId),
    ) {
        reg.set_all(initial, initial);
        line.set_all(connection, high, low, false).unwrap();
        reg.copy_from_line_state(&line.state());
        assert_eq!(reg.state().high, high | initial);
        assert_eq!(reg.state().low, low | initial);
    }
}
