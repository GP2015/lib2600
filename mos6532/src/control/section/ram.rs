use crate::{Riot, RiotError};
use emu_utils::pin::{BusCore, SinglePinCore, SinglePinOutput};

impl Riot {
    pub(crate) fn handle_ram(&mut self, only_possible: bool) -> Result<(), RiotError> {
        let rw_could_read_high = self.pin.rw.could_read_high();
        let rw_could_read_low = self.pin.rw.could_read_low();

        if rw_could_read_low {
            self.handle_read_ram(only_possible && !rw_could_read_high);
        }
        if rw_could_read_high {
            self.handle_write_ram(only_possible && !rw_could_read_low)?;
        }
        Ok(())
    }

    fn handle_read_ram(&mut self, only_possible: bool) {
        for address in self.pin.a.iter_possible_reads() {
            for (ram_bit, db_pin) in self
                .ram
                .byte_mut(address as u8)
                .iter_mut()
                .zip(self.pin.db.iter())
            {
                if only_possible {
                    ram_bit.set_all(false);
                }

                if db_pin.could_read_high() {
                    ram_bit.add(true);
                }

                if db_pin.could_read_low() {
                    ram_bit.add(false);
                }
            }
        }
    }

    fn handle_write_ram(&mut self, only_possible: bool) -> Result<(), RiotError> {
        for address in self.pin.a.iter_possible_reads() {
            for (ram_bit, db_pin) in self
                .ram
                .byte(address as u8)
                .iter()
                .zip(self.pin.db.iter_mut())
            {
                if only_possible {
                    db_pin.set_all_signals_out(false)?;
                }

                if ram_bit.could_read_high() {
                    db_pin.add_drive_in(true)?;
                }

                if ram_bit.could_read_low() {
                    db_pin.add_drive_in(false)?;
                }
            }
        }
        Ok(())
    }

    // pub(super) fn write_ram(&mut self) {
    //     for address in self.pin.a.iter_possible_reads() {
    //         for byte in self.pin.db.iter_possible_reads() {
    //             self.ram.byte_mut(address).add_wrapping(byte);
    //         }
    //     }
    // }

    // pub(super) fn read_ram(&mut self) {
    //     for address in self.pin.a.iter_possible_reads() {
    //         for byte in self.ram.byte(address).iter_possible_reads() {
    //             self.pin.db.add_drive_out_wrapping(byte);
    //         }
    //     }
    // }
}
