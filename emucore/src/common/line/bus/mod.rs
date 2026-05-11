pub mod state;

use crate::common::{
    bit,
    line::{
        bus::state::BusState,
        error::LineError,
        single::{Line, LineConId},
    },
    reg::mbit::state::MBitRegState,
};
use std::array;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BusConId(usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Bus<const SIZE: usize> {
    name: String,
    lines: [Line; SIZE],
    line_connections: Vec<[LineConId; SIZE]>,
}

macro_rules! line_con_row_iter {
    ($bus:ident, $bus_con:ident) => {
        $bus.line_connections
            .get($bus_con.0)
            .ok_or_else(|| LineError::ConnectionIdOutOfBounds {
                name: $bus.name.clone(),
            })?
            .iter()
            .copied()
    };
}

impl<const SIZE: usize> Bus<SIZE> {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();
        let lines = array::from_fn(|bit| Line::new(format!("{name}{bit}")));

        Self {
            name,
            lines,
            line_connections: Vec::new(),
        }
    }

    pub fn create_connection(&mut self) -> BusConId {
        let connection_row = array::from_fn(|bit| self.lines[bit].create_connection());

        self.line_connections.push(connection_row);
        BusConId(self.line_connections.len() - 1)
    }

    #[must_use]
    pub const fn name(&self) -> &str {
        self.name.as_str()
    }

    fn line_connection_row(&self, connection: BusConId) -> Result<&[LineConId; SIZE], LineError> {
        self.line_connections
            .get(connection.0)
            .ok_or_else(|| LineError::ConnectionIdOutOfBounds {
                name: self.name.clone(),
            })
    }

    #[must_use]
    pub const fn line<const BIT: usize>(&self) -> &Line {
        const { assert!(BIT < SIZE) }
        &self.lines[BIT]
    }

    pub fn line_mut<const BIT: usize>(
        &mut self,
        connection: BusConId,
    ) -> Result<(&mut Line, LineConId), LineError> {
        const { assert!(BIT < SIZE) }
        let connection = self.line_connection_row(connection)?[BIT];
        Ok((&mut self.lines[BIT], connection))
    }

    pub fn try_line(&self, bit: usize) -> Result<&Line, LineError> {
        self.lines.get(bit).ok_or_else(|| LineError::BitOutOfRange {
            name: self.name.clone(),
            bit,
            size: SIZE,
        })
    }

    pub fn try_line_mut(
        &mut self,
        connection: BusConId,
        bit: usize,
    ) -> Result<(&mut Line, LineConId), LineError> {
        if bit >= SIZE {
            return Err(LineError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: SIZE,
            });
        }
        let connection = self.line_connection_row(connection)?[bit];
        Ok((&mut self.lines[bit], connection))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    pub fn iter_mut(
        &mut self,
        connection: BusConId,
    ) -> Result<impl Iterator<Item = (&mut Line, LineConId)>, LineError> {
        let con_iter = line_con_row_iter!(self, connection);
        Ok(self.lines.iter_mut().zip(con_iter))
    }

    pub fn check_valid(&self) -> Result<(), LineError> {
        for line in self.iter() {
            line.check_valid()?;
        }
        Ok(())
    }

    #[must_use]
    pub fn state(&self) -> BusState<SIZE> {
        #[allow(clippy::indexing_slicing)]
        BusState::new(array::from_fn(|bit| self.lines[bit].state()))
    }

    pub fn add_high_z(&mut self, connection: BusConId) -> Result<(), LineError> {
        for (line, con) in self.iter_mut(connection)? {
            line.add_high_z(con)?;
        }
        Ok(())
    }

    pub fn add_drive_wrapping(
        &mut self,
        connection: BusConId,
        val: usize,
    ) -> Result<(), LineError> {
        let con_iter = line_con_row_iter!(self, connection);
        for (bit, (line, connection)) in self.lines.iter_mut().zip(con_iter).enumerate() {
            line.add_drive(connection, bit::bit_of_usize(val, bit))?;
        }
        Ok(())
    }

    pub fn add_drive(&mut self, connection: BusConId, val: usize) -> Result<(), LineError> {
        if bit::usize_exceeds_bit_count(val, self.lines.len()) {
            return Err(LineError::DriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.lines.len(),
            });
        }

        self.add_drive_wrapping(connection, bit::low_bits_of_usize(val, self.lines.len()))
    }

    pub fn remove_all(&mut self, connection: BusConId) -> Result<(), LineError> {
        for (line, con) in self.iter_mut(connection)? {
            line.remove_all(con)?;
        }
        Ok(())
    }

    pub fn copy_from_bus_state(
        &mut self,
        connection: BusConId,
        bus: &BusState<SIZE>,
    ) -> Result<(), LineError> {
        for ((this_line, line_connection), line_state) in self.iter_mut(connection)?.zip(bus.iter())
        {
            this_line.copy_from_line_state(line_connection, line_state)?;
        }

        Ok(())
    }

    pub fn copy_from_reg_state(
        &mut self,
        connection: BusConId,
        reg: &MBitRegState<SIZE>,
    ) -> Result<(), LineError> {
        for ((this_line, line_connection), bit_reg) in self.iter_mut(connection)?.zip(reg.iter()) {
            this_line.copy_from_reg_state(line_connection, bit_reg)?;
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::utils::line::LineError;
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
