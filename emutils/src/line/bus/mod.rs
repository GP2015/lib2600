pub mod state;

use crate::{
    bit,
    line::{Line, LineError, bus::state::BusState},
    reg::MBitRegister,
};
use itertools::Itertools;
use std::array;

#[derive(Debug)]
pub struct Bus<const N: usize> {
    name: String,
    lines: [Line; N],
    line_connections: Vec<[usize; N]>,
}

impl<const N: usize> Bus<N> {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();
        let lines = array::from_fn(|bit| Line::new(format!("{name}{bit}")));

        Self {
            name,
            lines,
            line_connections: Vec::new(),
        }
    }

    pub fn create_connection(&mut self) -> usize {
        let connection_row = array::from_fn(|bit| self.lines[bit].create_connection());
        self.line_connections.push(connection_row);
        self.line_connections.len() - 1
    }

    #[must_use]
    pub const fn name(&self) -> &str {
        self.name.as_str()
    }

    #[must_use]
    pub const fn size(&self) -> usize {
        self.lines.len()
    }

    fn check_for_bit_out_of_range(&self, bit: usize) -> Result<(), LineError> {
        if bit >= self.lines.len() {
            return Err(LineError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.lines.len(),
            });
        }
        Ok(())
    }

    pub fn line(&self, bit: usize) -> Result<&Line, LineError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&self.lines[bit])
    }

    pub fn line_mut(
        &mut self,
        connection: usize,
        bit: usize,
    ) -> Result<(&mut Line, usize), LineError> {
        self.check_for_bit_out_of_range(bit)?;
        let connection = self.line_connections[connection][bit];
        Ok((&mut self.lines[bit], connection))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    pub fn iter_mut(&mut self, connection: usize) -> impl Iterator<Item = (&mut Line, usize)> {
        self.lines
            .iter_mut()
            .zip(self.line_connections[connection].iter().copied())
    }

    #[must_use]
    pub fn state(&self) -> BusState<N> {
        BusState::new(array::from_fn(|bit| self.lines[bit].state()))
    }

    #[must_use]
    pub fn read(&self) -> Option<usize> {
        bit::some_bits_to_usize(self.lines.iter().map(Line::read))
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.lines
            .iter()
            .map(|line| line.possible_reads().iter().copied())
            .multi_cartesian_product()
            .map(|bits| bit::bits_to_usize(bits.into_iter()))
    }

    pub fn add_high_z(&mut self, connection: usize, only_possible: bool) {
        self.iter_mut(connection)
            .for_each(|(line, con)| line.add_high_z(con, only_possible));
    }

    pub fn add_drive_wrapping(
        &mut self,
        connection: usize,
        val: usize,
        only_possible: bool,
    ) -> Result<(), LineError> {
        for (bit, (line, connection)) in self
            .lines
            .iter_mut()
            .zip(self.line_connections[connection].iter().copied())
            .enumerate()
        {
            line.add_drive(connection, bit::bit_of_usize(val, bit), only_possible)?;
        }
        Ok(())
    }

    pub fn add_drive(
        &mut self,
        connection: usize,
        val: usize,
        only_possible: bool,
    ) -> Result<(), LineError> {
        if bit::usize_exceeds_bit_count(val, self.lines.len()) {
            return Err(LineError::DriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.lines.len(),
            });
        }

        self.add_drive_wrapping(
            connection,
            bit::low_bits_of_usize(val, self.lines.len()),
            only_possible,
        )
    }

    pub fn copy_from_bus(
        &mut self,
        connection: usize,
        bus: &Self,
        only_possible: bool,
    ) -> Result<(), LineError> {
        for ((this_line, line_connection), other_line) in self.iter_mut(connection).zip(bus.iter())
        {
            this_line.copy_from_line(line_connection, other_line, only_possible)?;
        }

        Ok(())
    }

    pub fn copy_from_reg(
        &mut self,
        connection: usize,
        reg: &MBitRegister<N>,
        only_possible: bool,
    ) -> Result<(), LineError> {
        for ((this_line, line_connection), bit_reg) in self.iter_mut(connection).zip(reg.iter()) {
            this_line.copy_from_reg(line_connection, bit_reg, only_possible)?;
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::line::LineError;
//     use rstest::{fixture, rstest};

//     type BusType = Bus;
//     const BUS_NAME: &str = "bus";

//     type DriveFn = fn(&mut BusType, usize, bool) -> Result<(), LineError>;

//     #[fixture]
//     fn bus() -> BusType {
//         Bus::new(BUS_NAME, 8)
//     }

//     #[rstest]
//     fn name(bus: BusType) {
//         assert_eq!(bus.name(), BUS_NAME);
//     }

//     #[rstest]
//     fn line_out_of_range(mut bus: BusType) {
//         assert!(matches!(
//             bus.line(8).err().unwrap(),
//             LineError::BitOutOfRange { .. }
//         ));
//         assert!(matches!(
//             bus.line_mut(9).err().unwrap(),
//             LineError::BitOutOfRange { .. }
//         ));
//     }

//     #[rstest]
//     fn read_success(mut bus: BusType) {
//         bus.add_drive(0x67, false).unwrap();
//         assert_eq!(bus.read().unwrap(), 0x67);
//         bus.post_tick_update();
//         bus.add_drive(0x89, false).unwrap();
//         assert_eq!(bus.read_prev().unwrap(), 0x67);
//     }

//     #[rstest]
//     fn drive(mut bus: BusType) {
//         bus.add_drive(0x67, false).unwrap();
//         assert_eq!(bus.read().unwrap(), 0x67);
//     }

//     #[rstest]
//     fn drive_too_large(mut bus: BusType) {
//         assert!(matches!(
//             bus.add_drive(0x167, false).err().unwrap(),
//             LineError::DriveValueTooLarge { .. }
//         ));
//     }

//     #[rstest]
//     #[case(0x67, 0x67)]
//     #[case(0x167, 0x67)]
//     fn wrapping_drive(mut bus: BusType, #[case] ival: usize, #[case] oval: usize) {
//         bus.add_drive_wrapping(ival, false).unwrap();
//         assert_eq!(bus.read().unwrap(), oval);
//     }
// }
