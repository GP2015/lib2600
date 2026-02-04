use crate::{Riot, RiotError};
use emu_utils::pin::BusOutput;

impl Riot {
    pub(super) fn read_interrupt_flag(&mut self) -> Result<(), RiotError> {
        let edc_interrupt_flag_usize = self.reg.edc_interrupt_flag.read()? as usize;
        let timer_flag_usize = self.reg.timer_flag.read()? as usize;
        let interrupt_reg = (edc_interrupt_flag_usize << 7) | (timer_flag_usize << 6);

        self.db_out_mut().drive_out(interrupt_reg)?;

        self.reg.edc_interrupt_flag.write(false);

        Ok(())
    }
}
