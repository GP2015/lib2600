use crate::{Riot, RiotError, pins::RiotLineRefs};

impl Riot {
    pub(crate) fn handle_ram(
        &mut self,
        lines: &mut RiotLineRefs,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        let rw_could_read_high = lines.rw.could_read_high();
        let rw_could_read_low = lines.rw.could_read_low();

        if rw_could_read_low {
            self.handle_write_ram(lines, only_possible && !rw_could_read_high)?;
        }

        if rw_could_read_high {
            self.handle_read_ram(lines, only_possible && !rw_could_read_low)?;
        }

        Ok(())
    }

    fn handle_write_ram(
        &mut self,
        lines: &mut RiotLineRefs,
        mut only_possible: bool,
    ) -> Result<(), RiotError> {
        only_possible &= lines.a.iter_possible_reads().count() == 1;

        for address in lines.a.iter_possible_reads() {
            self.ram
                .byte_mut(address as u8)
                .copy_from_bus(lines.db, only_possible)?;
        }
        Ok(())
    }

    fn handle_read_ram(
        &mut self,
        lines: &mut RiotLineRefs,
        mut only_possible: bool,
    ) -> Result<(), RiotError> {
        only_possible &= lines.a.iter_possible_reads().count() == 1;

        for address in lines.a.iter_possible_reads() {
            lines.db.copy_from_reg(
                self.connections.db,
                self.ram.byte(address as u8),
                only_possible,
            )?;
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::{fixture, rstest};

//     #[fixture]
//     pub fn riot() -> Riot {
//         let mut riot = Riot::new();
//         riot.reset_pulse().unwrap();
//         riot
//     }

//     #[rstest]
//     fn write_ram_defined_only_possible(mut riot: Riot) {
//         riot.pin.rs.add_drive_in(false, true).unwrap();
//         riot.pin.a.add_drive_in(0x67, true).unwrap();
//         riot.pin.db.add_drive_in(0x89, true).unwrap();
//         riot.handle_ram(true).unwrap();
//         assert_eq!(riot.ram.byte(0x67).read().unwrap(), 0x89);
//     }
// }
