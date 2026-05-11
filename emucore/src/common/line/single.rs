use crate::common::{
    line::{drive_state::DriveState, error::LineError},
    read::single::SingleRead,
    signal::LineSignal,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LineConId(usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Line {
    name: String,
    connection_states: Vec<DriveState>,
    combined_state: DriveState,
}

impl Line {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            connection_states: Vec::new(),
            combined_state: DriveState::default(),
        }
    }

    pub fn create_connection(&mut self) -> LineConId {
        let state = DriveState::default();
        self.connection_states.push(state);
        LineConId(self.connection_states.len() - 1)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn read(&self) -> SingleRead {
        self.combined_state.read()
    }

    fn contend_states(first: DriveState, second: DriveState) -> Option<DriveState> {
        let mut result = DriveState::default();

        let iter_possible = |state: DriveState| {
            [
                (state.low, LineSignal::Low),
                (state.high, LineSignal::High),
                (state.high_z, LineSignal::HighZ),
            ]
            .into_iter()
            .filter_map(|(enabled, signal)| enabled.then_some(signal))
        };

        for first_signal in iter_possible(first) {
            for second_signal in iter_possible(second) {
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
        let init = DriveState {
            low: false,
            high: false,
            high_z: true,
        };

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

    fn connection_state(&mut self, connection: LineConId) -> Result<&mut DriveState, LineError> {
        self.connection_states.get_mut(connection.0).ok_or_else(|| {
            LineError::ConnectionIdOutOfBounds {
                name: self.name.clone(),
            }
        })
    }

    pub fn set_drive_state(
        &mut self,
        connection: LineConId,
        state: DriveState,
    ) -> Result<(), LineError> {
        let old_state = self.connection_state(connection)?;
        *old_state = state;
        self.update_combined_state()
    }

    pub fn set_drive_to_read(
        &mut self,
        connection: LineConId,
        read: SingleRead,
    ) -> Result<(), LineError> {
        let state = self.connection_state(connection)?;
        state.low = matches!(read, SingleRead::Low | SingleRead::Unknown);
        state.high = matches!(read, SingleRead::High | SingleRead::Unknown);
        state.high_z = false;
        self.update_combined_state()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::{fixture, rstest};

//     const LINE_NAME: &str = "line";

//     #[fixture]
//     fn line() -> Line {
//         Line::new(LINE_NAME)
//     }

//     #[rstest]
//     fn new(line: Line) {
//         assert_eq!(line.name(), LINE_NAME);
//     }

//     #[rstest]
//     fn single_connection(mut line: Line) {
//         let connection = line.create_connection();
//         line.add_high(connection).unwrap();
//         assert!(line.state().read().unwrap());
//         line.add_low(connection).unwrap();
//         assert!(!line.state().read().unwrap());
//     }

//     #[rstest]
//     fn double_connection(mut line: Line) {
//         let connection1 = line.create_connection();
//         let connection2 = line.create_connection();
//         line.add_high(connection1).unwrap();
//         line.add_high_z(connection2).unwrap();
//         assert!(line.state().could_read_high());
//         assert!(!line.state().could_read_low());
//         line.add_high(connection2).unwrap();
//         assert!(line.state().could_read_high());
//         assert!(!line.state().could_read_low());
//         line.add_high_z(connection1).unwrap();
//         line.remove_high(connection2).unwrap();
//         assert!(line.state().could_read_high());
//         assert!(line.state().could_read_low());
//         line.add_low(connection1).unwrap();
//         assert!(!line.state().could_read_high());
//         assert!(line.state().could_read_low());
//     }
// }
