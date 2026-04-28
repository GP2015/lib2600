use crate::{
    bit,
    line::{BusConnection, Line, LineConnection, LineError},
    reg::MBitRegister,
};
use itertools::Itertools;

#[derive(Debug)]
pub struct Bus {
    name: String,
    lines: Box<[Line]>,
    line_connections: Vec<Box<[LineConnection]>>,
}

impl Bus {
    pub fn new<S: Into<String>>(name: S, size: usize) -> Self {
        let name = name.into();
        let lines = (0..size)
            .map(|bit| Line::new(format!("{name}{bit}")))
            .collect();

        Self {
            name,
            lines,
            line_connections: Vec::new(),
        }
    }

    pub fn create_connection(&mut self) -> BusConnection {
        let pin_connections = self.lines.iter_mut().map(Line::create_connection).collect();
        self.line_connections.push(pin_connections);
        BusConnection::new(self.line_connections.len() - 1)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[must_use]
    pub fn size(&self) -> usize {
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

    pub fn pin(&self, bit: usize) -> Result<&Line, LineError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&self.lines[bit])
    }

    pub fn pin_mut(
        &mut self,
        connection: BusConnection,
        bit: usize,
    ) -> Result<(&mut Line, &LineConnection), LineError> {
        self.check_for_bit_out_of_range(bit)?;
        let connection = &self.line_connections[connection.id()][bit];
        Ok((&mut self.lines[bit], connection))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    pub fn iter_mut(
        &mut self,
        connection: BusConnection,
    ) -> impl Iterator<Item = (&mut Line, &LineConnection)> {
        self.lines
            .iter_mut()
            .zip(self.line_connections[connection.id()].iter())
    }

    #[must_use]
    pub fn read(&self) -> Option<usize> {
        bit::some_bits_to_usize(self.lines.iter().map(Line::read))
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.lines
            .iter()
            .map(|pin| pin.possible_reads().iter().copied())
            .multi_cartesian_product()
            .map(|bits| bit::bits_to_usize(bits.into_iter()))
    }

    pub fn add_drive_wrapping(
        &mut self,
        connection: BusConnection,
        val: usize,
        only_possible: bool,
    ) -> Result<(), LineError> {
        for (bit, (line, connection)) in self
            .lines
            .iter_mut()
            .zip(self.line_connections[connection.id()].iter())
            .enumerate()
        {
            line.add_drive(connection, bit::bit_of_usize(val, bit), only_possible)?;
        }
        Ok(())
    }

    pub fn add_drive(
        &mut self,
        connection: BusConnection,
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
        connection: BusConnection,
        bus: &Bus,
        only_possible: bool,
    ) -> Result<(), LineError> {
        if self.size() != bus.size() {
            return Err(LineError::IncompatibleBus {
                out_name: self.name.clone(),
                out_size: self.size(),
                source_name: bus.name().to_string(),
                source_size: bus.size(),
            });
        }

        for ((this_line, line_connection), other_line) in self.iter_mut(connection).zip(bus.iter())
        {
            this_line.copy_from_line(line_connection, other_line, only_possible)?;
        }

        Ok(())
    }

    pub fn copy_from_reg(
        &mut self,
        connection: BusConnection,
        reg: &MBitRegister,
        only_possible: bool,
    ) -> Result<(), LineError> {
        if self.size() != reg.size() {
            return Err(LineError::IncompatibleRegister {
                bus_name: self.name.clone(),
                bus_size: self.size(),
                reg_name: reg.name().to_string(),
                reg_size: reg.size(),
            });
        }

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
//     fn pin_out_of_range(mut bus: BusType) {
//         assert!(matches!(
//             bus.pin(8).err().unwrap(),
//             LineError::BitOutOfRange { .. }
//         ));
//         assert!(matches!(
//             bus.pin_mut(9).err().unwrap(),
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
