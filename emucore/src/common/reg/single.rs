use crate::common::read::single::SingleRead;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BitReg {
    inner: SingleRead,
}

impl BitReg {
    pub const fn new(initial: SingleRead) -> Self {
        Self { inner: initial }
    }

    pub const fn read(&self) -> SingleRead {
        self.inner
    }

    pub const fn set_to_read(&mut self, inner: SingleRead) {
        self.inner = inner;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::common::line::single::{Line, LineConId};
//     use rstest::{fixture, rstest};

//     const REG_NAME: &str = "reg";

//     #[fixture]
//     fn line_and_connection() -> (Line, LineConId) {
//         let mut line = Line::new("");
//         let connection = line.create_connection();
//         (line, connection)
//     }

//     #[fixture]
//     fn reg() -> BitReg {
//         BitReg::new(REG_NAME, true, true)
//     }

//     #[rstest]
//     fn initial(reg: BitReg) {
//         assert!(reg.state().low);
//         assert!(reg.state().high);
//     }

//     #[rstest]
//     fn name(reg: BitReg) {
//         assert_eq!(reg.name(), REG_NAME);
//     }

//     #[rstest]
//     fn copy_from_line_only_possible(
//         #[values(true, false)] initial: bool,
//         #[values(true, false)] low: bool,
//         #[values(true, false)] high: bool,
//         mut reg: BitReg,
//         #[from(line_and_connection)] (mut line, connection): (Line, LineConId),
//     ) {
//         reg.set_all(initial, initial);
//         line.set_all(connection, high, low, false).unwrap();
//         reg.copy_from_line_state(line.state());
//         assert_eq!(reg.state().high, high);
//         assert_eq!(reg.state().low, low);
//     }

//     #[rstest]
//     fn copy_from_line_not_only_possible(
//         #[values(true, false)] initial: bool,
//         #[values(true, false)] low: bool,
//         #[values(true, false)] high: bool,
//         mut reg: BitReg,
//         #[from(line_and_connection)] (mut line, connection): (Line, LineConId),
//     ) {
//         reg.set_all(initial, initial);
//         line.set_all(connection, high, low, false).unwrap();
//         reg.copy_from_line_state(line.state());
//         assert_eq!(reg.state().high, high | initial);
//         assert_eq!(reg.state().low, low | initial);
//     }
// }
