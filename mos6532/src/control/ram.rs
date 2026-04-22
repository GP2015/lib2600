use crate::{Riot, RiotError};
use emutils::pin::{BusInputUI, BusOutput, PinInputUI};

impl Riot {
    pub(crate) fn handle_ram(&mut self, only_possible: bool) -> Result<(), RiotError> {
        let rw_could_read_high = self.pin.rw.could_read_high();
        let rw_could_read_low = self.pin.rw.could_read_low();

        if rw_could_read_low {
            self.handle_write_ram(only_possible && !rw_could_read_high);
        }

        if rw_could_read_high {
            self.handle_read_ram(only_possible && !rw_could_read_low)?;
        }

        Ok(())
    }

    fn handle_write_ram(&mut self, mut only_possible: bool) {
        only_possible &= self.pin.a.iter_possible_reads().count() == 1;

        for address in self.pin.a.iter_possible_reads() {
            self.ram
                .byte_mut(address as u8)
                .input_from_bus(&self.pin.db, only_possible);
        }
    }

    fn handle_read_ram(&mut self, mut only_possible: bool) -> Result<(), RiotError> {
        only_possible &= self.pin.a.iter_possible_reads().count() == 1;

        for address in self.pin.a.iter_possible_reads() {
            self.pin
                .db
                .output_from_reg(self.ram.byte(address as u8), only_possible)?;
        }
        Ok(())
    }
}
