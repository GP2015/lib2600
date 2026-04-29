use crate::{Riot, RiotError, RiotLineRefs};

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
            lines
                .db
                .copy_from_reg(self.db_con, self.ram.byte(address as u8), only_possible)?;
        }
        Ok(())
    }
}
