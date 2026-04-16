use crate::{Riot, RiotError};

impl Riot {
    pub(crate) fn handle_read_interrupt_flag(
        &mut self,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        Ok(())
    }

    // pub(super) fn read_interrupt_flag(&mut self) -> Result<(), RiotError> {
    //     let edc_interrupt_flag_usize = self.reg.edc_interrupt_flag.read()? as usize;
    //     let timer_flag_usize = self.reg.timer_flag.read()? as usize;
    //     let interrupt_reg = (edc_interrupt_flag_usize << 7) | (timer_flag_usize << 6);

    //     self.db_out().drive_out(interrupt_reg)?;

    //     self.reg.edc_interrupt_flag.write(false);

    //     Ok(())
    // }
}
