pub mod state;

use crate::common::{
    line::{error::LineError, signal::LineSignal, single::state::LineState},
    reg::bit::state::BitRegState,
};
use delegate::delegate;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LineConId(usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Line {
    name: String,
    connection_states: Vec<LineState>,
    combined_state: LineState,
}

impl Line {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            connection_states: Vec::new(),
            combined_state: LineState::new(false, false, false),
        }
    }

    pub fn create_connection(&mut self) -> LineConId {
        let state = LineState::new(false, false, false);
        self.connection_states.push(state);
        LineConId(self.connection_states.len() - 1)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn state(&self) -> LineState {
        self.combined_state
    }

    #[must_use]
    fn contend_states(first: LineState, second: LineState) -> Option<LineState> {
        let mut result = LineState::new(false, false, false);

        for first_signal in first.iter_possible() {
            for second_signal in second.iter_possible() {
                match first_signal.contend_with(second_signal)? {
                    LineSignal::Low => result.low = true,
                    LineSignal::High => result.high = true,
                    LineSignal::HighZ => result.high_z = true,
                }
            }
        }

        Some(result)
    }

    fn update_combined_state(&mut self) -> Result<(), LineError> {
        let init = LineState::new(false, false, true);
        self.combined_state = self
            .connection_states
            .iter()
            .copied()
            .try_fold(init, Self::contend_states)
            .ok_or_else(|| LineError::ShortCircuit {
                name: self.name().to_string(),
            })?;
        Ok(())
    }

    pub fn check_valid(&self) -> Result<(), LineError> {
        if !self.combined_state.is_valid() {
            return Err(LineError::ImpossibleLineSignal {
                name: self.name.clone(),
            });
        }
        Ok(())
    }

    fn connection_state_mut(&mut self, connection: LineConId) -> Result<&mut LineState, LineError> {
        self.connection_states.get_mut(connection.0).ok_or_else(|| {
            LineError::ConnectionIdOutOfBounds {
                name: self.name.clone(),
            }
        })
    }

    pub fn remove_all(&mut self, connection: LineConId) -> Result<(), LineError> {
        self.set_all(connection, false, false, false)?;
        self.update_combined_state().unwrap();
        Ok(())
    }

    pub fn add(&mut self, connection: LineConId, signal: LineSignal) -> Result<(), LineError> {
        let state = self.connection_state_mut(connection)?;
        match signal {
            LineSignal::Low => state.low = true,
            LineSignal::High => state.high = true,
            LineSignal::HighZ => state.high_z = true,
        }
        self.update_combined_state()
    }

    pub fn remove(&mut self, connection: LineConId, signal: LineSignal) -> Result<(), LineError> {
        let state = self.connection_state_mut(connection)?;
        match signal {
            LineSignal::Low => state.low = false,
            LineSignal::High => state.high = false,
            LineSignal::HighZ => state.high_z = false,
        }
        self.update_combined_state()
    }

    pub fn set_all(
        &mut self,
        connection: LineConId,
        low: bool,
        high: bool,
        high_z: bool,
    ) -> Result<(), LineError> {
        let state = self.connection_state_mut(connection)?;
        state.low = low;
        state.high = high;
        state.high_z = high_z;
        self.update_combined_state()
    }

    pub fn add_drive(&mut self, connection: LineConId, val: bool) -> Result<(), LineError> {
        if val {
            self.add_high(connection)
        } else {
            self.add_low(connection)
        }
    }

    pub fn remove_drive(&mut self, connection: LineConId, val: bool) -> Result<(), LineError> {
        if val {
            self.remove_high(connection)
        } else {
            self.remove_low(connection)
        }
    }

    pub fn copy_from_line_state(
        &mut self,
        connection: LineConId,
        line_state: LineState,
    ) -> Result<(), LineError> {
        let state = self.connection_state_mut(connection)?;

        if line_state.low {
            state.low = true;
        }

        if line_state.high {
            state.high = true;
        }

        if line_state.high_z {
            state.high_z = true;
        }

        self.update_combined_state()
    }

    pub fn copy_from_reg_state(
        &mut self,
        connection: LineConId,
        reg_state: BitRegState,
    ) -> Result<(), LineError> {
        let state = self.connection_state_mut(connection)?;

        if reg_state.low {
            state.low = true;
        }

        if reg_state.high {
            state.high = true;
        }

        self.update_combined_state()
    }

    delegate! {
        to self{
            #[call(add)]
            pub fn add_low(&mut self, connection: LineConId, [LineSignal::Low]) -> Result<(), LineError>;
            #[call(add)]
            pub fn add_high(&mut self, connection: LineConId, [LineSignal::High]) -> Result<(), LineError>;
            #[call(add)]
            pub fn add_high_z(&mut self, connection: LineConId, [LineSignal::HighZ]) -> Result<(), LineError>;

            #[call(remove)]
            pub fn remove_low(&mut self, connection: LineConId, [LineSignal::Low]) -> Result<(), LineError>;
            #[call(remove)]
            pub fn remove_high(&mut self, connection: LineConId, [LineSignal::High]) -> Result<(), LineError>;
            #[call(remove)]
            pub fn remove_high_z(&mut self, connection: LineConId, [LineSignal::HighZ]) -> Result<(), LineError>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    const LINE_NAME: &str = "line";

    #[fixture]
    fn line() -> Line {
        Line::new(LINE_NAME)
    }

    #[rstest]
    fn new(line: Line) {
        assert_eq!(line.name(), LINE_NAME);
    }

    #[rstest]
    fn single_connection(mut line: Line) {
        let connection = line.create_connection();
        line.add_high(connection).unwrap();
        assert!(line.state().read().unwrap());
        line.add_low(connection).unwrap();
        assert!(!line.state().read().unwrap());
    }

    #[rstest]
    fn double_connection(mut line: Line) {
        let connection1 = line.create_connection();
        let connection2 = line.create_connection();
        line.add_high(connection1).unwrap();
        line.add_high_z(connection2).unwrap();
        assert!(line.state().could_read_high());
        assert!(!line.state().could_read_low());
        line.add_high(connection2).unwrap();
        assert!(line.state().could_read_high());
        assert!(!line.state().could_read_low());
        line.add_high_z(connection1).unwrap();
        line.remove_high(connection2).unwrap();
        assert!(line.state().could_read_high());
        assert!(line.state().could_read_low());
        line.add_low(connection1).unwrap();
        assert!(!line.state().could_read_high());
        assert!(line.state().could_read_low());
    }
}
